// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`FontChooser`](crate::FontChooser) interface.

use crate::{prelude::*, subclass::prelude::*, FontChooser};
use glib::{translate::*, GString, Quark};
use once_cell::sync::Lazy;
use pango::{FontFace, FontFamily, FontMap};

use super::PtrHolder;

#[derive(Debug)]
pub struct FilterCallback {
    filter_func: ffi::GtkFontFilterFunc,
    user_data: glib::ffi::gpointer,
    destroy_notify: glib::ffi::GDestroyNotify,
}

impl FilterCallback {
    pub fn call(&self, font_family: &FontFamily, font_face: &FontFace) -> bool {
        unsafe {
            if let Some(filter_func) = self.filter_func {
                from_glib(filter_func(
                    font_family.to_glib_none().0,
                    font_face.to_glib_none().0,
                    self.user_data,
                ))
            } else {
                // show the font if the filter_func was not set
                true
            }
        }
    }
}

impl Drop for FilterCallback {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if let Some(destroy_notify) = self.destroy_notify {
                destroy_notify(self.user_data)
            }
        }
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait FontChooserImpl: ObjectImpl {
    fn font_family(&self) -> Option<FontFamily> {
        self.parent_font_family()
    }

    fn font_face(&self) -> Option<FontFace> {
        self.parent_font_face()
    }

    fn font_size(&self) -> i32 {
        self.parent_font_size()
    }

    fn set_filter_func(&self, callback: Option<FilterCallback>) {
        self.parent_set_filter_func(callback)
    }

    fn set_font_map<P: IsA<FontMap>>(&self, font_map: Option<&P>) {
        self.parent_set_font_map(font_map)
    }

    fn font_map(&self) -> Option<FontMap> {
        self.parent_font_map()
    }

    fn font_activated(&self, font_name: &str) {
        self.parent_font_activated(font_name)
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait FontChooserImplExt: ObjectSubclass {
    fn parent_font_family(&self) -> Option<FontFamily>;
    fn parent_font_face(&self) -> Option<FontFace>;
    fn parent_font_size(&self) -> i32;
    fn parent_set_filter_func(&self, callback: Option<FilterCallback>);
    fn parent_set_font_map<P: IsA<FontMap>>(&self, font_map: Option<&P>);
    fn parent_font_map(&self) -> Option<FontMap>;
    fn parent_font_activated(&self, font_name: &str);
}

impl<O: FontChooserImpl> FontChooserImplExt for O {
    fn parent_font_family(&self) -> Option<FontFamily> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;
            let f = (*parent_iface)
                .get_font_family
                .expect("no parent \"get_font_family\" implementation");

            from_glib_none(f(self
                .obj()
                .unsafe_cast_ref::<FontChooser>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_font_face(&self) -> Option<FontFace> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;

            let f = (*parent_iface)
                .get_font_face
                .expect("no parent \"get_font_face\" implementation");

            from_glib_none(f(self
                .obj()
                .unsafe_cast_ref::<FontChooser>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_font_size(&self) -> i32 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;

            if let Some(f) = (*parent_iface).get_font_size {
                f(self.obj().unsafe_cast_ref::<FontChooser>().to_glib_none().0)
            } else {
                // No font size is selected
                -1
            }
        }
    }

    fn parent_set_filter_func(&self, callback: Option<FilterCallback>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;

            if let Some(f) = (*parent_iface).set_filter_func {
                if let Some(filter_callback) = callback {
                    f(
                        self.obj().unsafe_cast_ref::<FontChooser>().to_glib_none().0,
                        filter_callback.filter_func,
                        filter_callback.user_data,
                        filter_callback.destroy_notify,
                    )
                } else {
                    f(
                        self.obj().unsafe_cast_ref::<FontChooser>().to_glib_none().0,
                        None,
                        std::ptr::null_mut(),
                        None,
                    )
                }
            }
        }
    }

    fn parent_set_font_map<P: IsA<FontMap>>(&self, font_map: Option<&P>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;

            let f = (*parent_iface)
                .set_font_map
                .expect("no parent \"set_font_map\" implementation");
            f(
                self.obj().unsafe_cast_ref::<FontChooser>().to_glib_none().0,
                font_map.map(|fm| fm.as_ref()).to_glib_none().0,
            );
        }
    }

    fn parent_font_map(&self) -> Option<FontMap> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;

            let f = (*parent_iface)
                .get_font_map
                .expect("no parent \"get_font_map\" implementation");

            from_glib_none(f(self
                .obj()
                .unsafe_cast_ref::<FontChooser>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_font_activated(&self, font_name: &str) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;
            if let Some(f) = (*parent_iface).font_activated {
                f(
                    self.obj().unsafe_cast_ref::<FontChooser>().to_glib_none().0,
                    font_name.to_glib_none().0,
                );
            }
        }
    }
}

unsafe impl<T: FontChooserImpl> IsImplementable<T> for FontChooser {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        assert_initialized_main_thread!();

        iface.get_font_family = Some(font_chooser_get_font_family::<T>);
        iface.get_font_face = Some(font_chooser_get_font_face::<T>);
        iface.get_font_size = Some(font_chooser_get_font_size::<T>);
        iface.font_activated = Some(font_chooser_font_activated::<T>);
        iface.set_font_map = Some(font_chooser_set_font_map::<T>);
        iface.get_font_map = Some(font_chooser_get_font_map::<T>);
        iface.set_filter_func = Some(font_chooser_set_filter_func::<T>);
    }
}

static FONT_CHOOSER_GET_FONT_FAMILY_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_str("gtk4-rs-subclass-font-chooser-font-family"));

unsafe extern "C" fn font_chooser_get_font_family<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
) -> *mut pango::ffi::PangoFontFamily {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.imp();

    let ret = imp.font_family();
    if let Some(font_family) = ret {
        let font_family = font_family.into_glib_ptr();
        imp.obj().set_qdata(
            *FONT_CHOOSER_GET_FONT_FAMILY_QUARK,
            PtrHolder(font_family, |ptr| {
                glib::gobject_ffi::g_object_unref(ptr as *mut _)
            }),
        );
        font_family
    } else {
        std::ptr::null_mut()
    }
}

static FONT_CHOOSER_GET_FONT_FACE_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_str("gtk4-rs-subclass-font-chooser-font-face"));
unsafe extern "C" fn font_chooser_get_font_face<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
) -> *mut pango::ffi::PangoFontFace {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.imp();

    let ret = imp.font_face();
    if let Some(font_face) = ret {
        let font_face = font_face.into_glib_ptr();
        imp.obj().set_qdata(
            *FONT_CHOOSER_GET_FONT_FACE_QUARK,
            PtrHolder(font_face, |ptr| {
                glib::gobject_ffi::g_object_unref(ptr as *mut _);
            }),
        );
        font_face
    } else {
        std::ptr::null_mut()
    }
}

unsafe extern "C" fn font_chooser_get_font_size<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
) -> i32 {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.imp();

    imp.font_size()
}

unsafe extern "C" fn font_chooser_font_activated<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
    font_nameptr: *const libc::c_char,
) {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.imp();
    let font_name: Borrowed<GString> = from_glib_borrow(font_nameptr);

    imp.font_activated(&font_name)
}

unsafe extern "C" fn font_chooser_set_font_map<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
    font_mapptr: *mut pango::ffi::PangoFontMap,
) {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.imp();
    let font_map: Borrowed<Option<FontMap>> = from_glib_borrow(font_mapptr);

    imp.set_font_map(font_map.as_ref().as_ref())
}

unsafe extern "C" fn font_chooser_get_font_map<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
) -> *mut pango::ffi::PangoFontMap {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.imp();

    imp.font_map().into_glib_ptr()
}

unsafe extern "C" fn font_chooser_set_filter_func<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
    filter_func: ffi::GtkFontFilterFunc,
    user_data: glib::ffi::gpointer,
    destroy_notify: glib::ffi::GDestroyNotify,
) {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.imp();

    let callback = if filter_func.is_some() {
        None
    } else {
        Some(FilterCallback {
            filter_func,
            user_data,
            destroy_notify,
        })
    };

    imp.set_filter_func(callback);
}
