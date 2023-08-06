// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Win32Display, Win32MessageFilterReturn, MSG};
use glib::translate::*;

#[cfg(all(feature = "v4_4", feature = "egl"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "v4_4", feature = "egl"))))]
use khronos_egl as egl;

impl Win32Display {
    #[cfg(all(feature = "v4_4", feature = "egl"))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "v4_4", feature = "egl"))))]
    #[doc(alias = "gdk_win32_display_get_egl_display")]
    #[doc(alias = "get_egl_display")]
    pub fn egl_display(&self) -> Option<egl::Display> {
        unsafe {
            let ptr = ffi::gdk_win32_display_get_egl_display(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(egl::Display::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "gdk_win32_display_add_filter")]
    pub fn add_filter<F>(&self, filter_func: F) -> Win32DisplayFilterHandle
    where
        F: Fn(&Self, &mut MSG, &mut i32) -> Win32MessageFilterReturn + 'static,
    {
        unsafe extern "C" fn trampoline<
            F: Fn(&Win32Display, &mut MSG, &mut i32) -> Win32MessageFilterReturn + 'static,
        >(
            display: *mut ffi::GdkWin32Display,
            msg: glib::ffi::gpointer,
            return_value: *mut libc::c_int,
            box_: glib::ffi::gpointer,
        ) -> i32 {
            let f: &F = &*(box_ as *const F);
            f(
                &from_glib_borrow(display as *mut ffi::GdkWin32Display),
                &mut *(msg as *mut MSG),
                &mut *return_value,
            )
            .into_glib()
        }

        let box_ = Box::into_raw(Box::new(filter_func)) as *mut _;
        let func = unsafe {
            let func: ffi::GdkWin32MessageFilterFunc = Some(trampoline::<F>);
            ffi::gdk_win32_display_add_filter(self.to_glib_none().0, func, box_);
            func
        };

        let display = glib::WeakRef::new();
        display.set(Some(self));

        let drop_ = |b| unsafe {
            let _ = Box::<F>::from_raw(b as *mut _);
        };
        Win32DisplayFilterHandle {
            display,
            func,
            box_,
            drop_,
        }
    }
}

// rustdoc-stripper-ignore-next
/// An owned `Win32Display` filter.
///
/// A `Win32DisplayFilterHandle` removes itself from the `Win32Display` it is
/// attached to when it is dropped.
#[derive(Debug)]
pub struct Win32DisplayFilterHandle {
    display: glib::WeakRef<Win32Display>,
    func: ffi::GdkWin32MessageFilterFunc,
    box_: glib::ffi::gpointer,
    drop_: fn(*mut libc::c_void),
}

impl Drop for Win32DisplayFilterHandle {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if let Some(display) = self.display.upgrade() {
                ffi::gdk_win32_display_remove_filter(
                    display.to_glib_none().0,
                    self.func,
                    self.box_,
                );
            }
            (self.drop_)(self.box_);
        }
    }
}
