// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Display, Key, KeymapKey, ModifierType};
use glib::{translate::*, IntoGStr};
use std::{mem, ptr};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Backend {
    Wayland,
    X11,
    Win32,
    MacOS,
    Broadway,
}

impl Backend {
    // rustdoc-stripper-ignore-next
    /// Equivalent to the C macro `GDK_IS_WAYLAND_DISPLAY`
    #[doc(alias = "GDK_IS_WAYLAND_DISPLAY")]
    pub fn is_wayland(&self) -> bool {
        matches!(self, Self::Wayland)
    }

    // rustdoc-stripper-ignore-next
    /// Equivalent to the C macro `GDK_IS_X11_DISPLAY`
    #[doc(alias = "GDK_IS_X11_DISPLAY")]
    pub fn is_x11(&self) -> bool {
        matches!(self, Self::X11)
    }

    // rustdoc-stripper-ignore-next
    /// Equivalent to the C macro `GDK_IS_WIN32_DISPLAY`
    #[doc(alias = "GDK_IS_WIN32_DISPLAY")]
    pub fn is_win32(&self) -> bool {
        matches!(self, Self::Win32)
    }

    // rustdoc-stripper-ignore-next
    /// Equivalent to the C macro `GDK_IS_MACOS_DISPLAY`
    #[doc(alias = "GDK_IS_MACOS_DISPLAY")]
    pub fn is_macos(&self) -> bool {
        matches!(self, Self::MacOS)
    }

    // rustdoc-stripper-ignore-next
    /// Equivalent to the C macro `GDK_IS_BROADWAY_DISPLAY`
    #[doc(alias = "GDK_IS_BROADWAY_DISPLAY")]
    pub fn is_broadway(&self) -> bool {
        matches!(self, Self::Broadway)
    }
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Display`](crate::Display).
pub trait DisplayExtManual: 'static {
    #[doc(alias = "gdk_display_translate_key")]
    fn translate_key(
        &self,
        keycode: u32,
        state: ModifierType,
        group: i32,
    ) -> Option<(Key, i32, i32, ModifierType)>;

    #[doc(alias = "gdk_display_get_setting")]
    fn get_setting(&self, name: impl IntoGStr) -> Option<glib::Value>;

    #[doc(alias = "gdk_display_map_keyval")]
    fn map_keyval(&self, keyval: Key) -> Option<Vec<KeymapKey>>;

    #[doc(alias = "gdk_display_map_keycode")]
    fn map_keycode(&self, keycode: u32) -> Option<Vec<(KeymapKey, Key)>>;

    // rustdoc-stripper-ignore-next
    /// Get the currently used display backend
    fn backend(&self) -> Backend;
}

impl<O: IsA<Display>> DisplayExtManual for O {
    fn translate_key(
        &self,
        keycode: u32,
        state: ModifierType,
        group: i32,
    ) -> Option<(Key, i32, i32, ModifierType)> {
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let mut effective_group = mem::MaybeUninit::uninit();
            let mut level = mem::MaybeUninit::uninit();
            let mut consumed = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_display_translate_key(
                self.as_ref().to_glib_none().0,
                keycode,
                state.into_glib(),
                group,
                keyval.as_mut_ptr(),
                effective_group.as_mut_ptr(),
                level.as_mut_ptr(),
                consumed.as_mut_ptr(),
            ));
            if ret {
                let keyval = keyval.assume_init();
                let effective_group = effective_group.assume_init();
                let level = level.assume_init();
                let consumed = consumed.assume_init();
                Some((
                    from_glib(keyval),
                    effective_group,
                    level,
                    from_glib(consumed),
                ))
            } else {
                None
            }
        }
    }

    fn get_setting(&self, name: impl IntoGStr) -> Option<glib::Value> {
        unsafe {
            name.run_with_gstr(|name| {
                let mut value = glib::Value::uninitialized();
                let ret = ffi::gdk_display_get_setting(
                    self.as_ref().to_glib_none().0,
                    name.as_ptr(),
                    value.to_glib_none_mut().0,
                );
                if from_glib(ret) {
                    Some(value)
                } else {
                    None
                }
            })
        }
    }

    fn map_keyval(&self, keyval: Key) -> Option<Vec<KeymapKey>> {
        unsafe {
            let mut keys = ptr::null_mut();
            let mut n_keys = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_display_map_keyval(
                self.as_ref().to_glib_none().0,
                keyval.into_glib(),
                &mut keys,
                n_keys.as_mut_ptr(),
            ));
            if ret {
                Some(FromGlibContainer::from_glib_full_num(
                    keys,
                    n_keys.assume_init() as usize,
                ))
            } else {
                None
            }
        }
    }

    fn map_keycode(&self, keycode: u32) -> Option<Vec<(KeymapKey, Key)>> {
        unsafe {
            let mut keys = ptr::null_mut();
            let mut keyvals = ptr::null_mut();
            let mut n_entries = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_display_map_keycode(
                self.as_ref().to_glib_none().0,
                keycode,
                &mut keys,
                &mut keyvals,
                n_entries.as_mut_ptr(),
            ));
            if ret {
                let n_keys = n_entries.assume_init() as usize;
                let keyvals: Vec<u32> = FromGlibContainer::from_glib_full_num(keyvals, n_keys);
                let keyvals = keyvals.into_iter().map(|k| from_glib(k));
                let keys: Vec<KeymapKey> = FromGlibContainer::from_glib_full_num(keys, n_keys);

                Some(keys.into_iter().zip(keyvals).collect())
            } else {
                None
            }
        }
    }

    fn backend(&self) -> Backend {
        match self.as_ref().type_().name() {
            "GdkWaylandDisplay" => Backend::Wayland,
            "GdkX11Display" => Backend::X11,
            "GdkMacosDisplay" => Backend::MacOS,
            "GdkWin32Display" => Backend::Win32,
            "GdkBroadwayDisplay" => Backend::Broadway,
            e => panic!("Unsupported display backend {e}"),
        }
    }
}
