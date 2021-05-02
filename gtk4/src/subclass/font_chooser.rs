// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::FontChooser;
use glib::translate::*;
use glib::{Cast, GString, IsA, ObjectExt, Quark};
use once_cell::sync::Lazy;
use pango::{FontFace, FontFamily, FontMap};

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
    fn drop(&mut self) {
        unsafe {
            if let Some(destroy_notify) = self.destroy_notify {
                destroy_notify(self.user_data)
            }
        }
    }
}

pub trait FontChooserImpl: ObjectImpl {
    fn font_family(&self, font_chooser: &Self::Type) -> Option<FontFamily> {
        self.parent_font_family(font_chooser)
    }

    fn font_face(&self, font_chooser: &Self::Type) -> Option<FontFace> {
        self.parent_font_face(font_chooser)
    }

    fn font_size(&self, font_chooser: &Self::Type) -> i32 {
        self.parent_font_size(font_chooser)
    }

    fn set_filter_func(&self, font_chooser: &Self::Type, callback: Option<FilterCallback>) {
        self.parent_set_filter_func(font_chooser, callback)
    }

    fn set_font_map<P: IsA<FontMap>>(&self, font_chooser: &Self::Type, font_map: Option<&P>) {
        self.parent_set_font_map(font_chooser, font_map)
    }

    fn font_map(&self, font_chooser: &Self::Type) -> Option<FontMap> {
        self.parent_font_map(font_chooser)
    }

    fn font_activated(&self, font_chooser: &Self::Type, font_name: &str) {
        self.parent_font_activated(font_chooser, font_name)
    }
}

pub trait FontChooserImplExt: ObjectSubclass {
    fn parent_font_family(&self, font_chooser: &Self::Type) -> Option<FontFamily>;
    fn parent_font_face(&self, font_chooser: &Self::Type) -> Option<FontFace>;
    fn parent_font_size(&self, font_chooser: &Self::Type) -> i32;
    fn parent_set_filter_func(&self, font_chooser: &Self::Type, callback: Option<FilterCallback>);
    fn parent_set_font_map<P: IsA<FontMap>>(&self, font_chooser: &Self::Type, font_map: Option<&P>);
    fn parent_font_map(&self, font_chooser: &Self::Type) -> Option<FontMap>;
    fn parent_font_activated(&self, font_chooser: &Self::Type, font_name: &str);
}

impl<O: FontChooserImpl> FontChooserImplExt for O {
    fn parent_font_family(&self, font_chooser: &Self::Type) -> Option<FontFamily> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;
            let f = (*parent_iface)
                .get_font_family
                .expect("no parent \"get_font_family\" implementation");

            from_glib_none(f(font_chooser
                .unsafe_cast_ref::<FontChooser>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_font_face(&self, font_chooser: &Self::Type) -> Option<FontFace> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;

            let f = (*parent_iface)
                .get_font_face
                .expect("no parent \"get_font_face\" implementation");

            from_glib_none(f(font_chooser
                .unsafe_cast_ref::<FontChooser>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_font_size(&self, font_chooser: &Self::Type) -> i32 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;

            if let Some(f) = (*parent_iface).get_font_size {
                f(font_chooser
                    .unsafe_cast_ref::<FontChooser>()
                    .to_glib_none()
                    .0)
            } else {
                // No font size is selected
                -1
            }
        }
    }

    fn parent_set_filter_func(&self, font_chooser: &Self::Type, callback: Option<FilterCallback>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;

            if let Some(f) = (*parent_iface).set_filter_func {
                if let Some(filter_callback) = callback {
                    f(
                        font_chooser
                            .unsafe_cast_ref::<FontChooser>()
                            .to_glib_none()
                            .0,
                        filter_callback.filter_func,
                        filter_callback.user_data,
                        filter_callback.destroy_notify,
                    )
                } else {
                    f(
                        font_chooser
                            .unsafe_cast_ref::<FontChooser>()
                            .to_glib_none()
                            .0,
                        None,
                        std::ptr::null_mut(),
                        None,
                    )
                }
            }
        }
    }

    fn parent_set_font_map<P: IsA<FontMap>>(
        &self,
        font_chooser: &Self::Type,
        font_map: Option<&P>,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;

            let f = (*parent_iface)
                .set_font_map
                .expect("no parent \"set_font_map\" implementation");
            f(
                font_chooser
                    .unsafe_cast_ref::<FontChooser>()
                    .to_glib_none()
                    .0,
                font_map.map(|fm| fm.as_ref()).to_glib_none().0,
            );
        }
    }

    fn parent_font_map(&self, font_chooser: &Self::Type) -> Option<FontMap> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;

            let f = (*parent_iface)
                .get_font_map
                .expect("no parent \"get_font_map\" implementation");

            from_glib_none(f(font_chooser
                .unsafe_cast_ref::<FontChooser>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_font_activated(&self, font_chooser: &Self::Type, font_name: &str) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<FontChooser>()
                as *const ffi::GtkFontChooserIface;
            if let Some(f) = (*parent_iface).font_activated {
                f(
                    font_chooser
                        .unsafe_cast_ref::<FontChooser>()
                        .to_glib_none()
                        .0,
                    font_name.to_glib_none().0,
                );
            }
        }
    }
}

unsafe impl<T: FontChooserImpl> IsImplementable<T> for FontChooser {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_font_family = Some(font_chooser_get_font_family::<T>);
        iface.get_font_face = Some(font_chooser_get_font_face::<T>);
        iface.get_font_size = Some(font_chooser_get_font_size::<T>);
        iface.font_activated = Some(font_chooser_font_activated::<T>);
        iface.set_font_map = Some(font_chooser_set_font_map::<T>);
        iface.get_font_map = Some(font_chooser_get_font_map::<T>);
        iface.set_filter_func = Some(font_chooser_set_filter_func::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

static FONT_CHOOSER_GET_FONT_FAMILY_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk4-rs-subclass-font-chooser-font-family"));

#[derive(Debug)]
struct FontFamilyWrapper(*mut pango::ffi::PangoFontFamily);

impl Drop for FontFamilyWrapper {
    fn drop(&mut self) {
        unsafe { glib::gobject_ffi::g_object_unref(self.0 as *mut _) }
    }
}

unsafe extern "C" fn font_chooser_get_font_family<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
) -> *mut pango::ffi::PangoFontFamily {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.impl_();
    let wrap = from_glib_borrow::<_, FontChooser>(font_chooser);

    let ret = imp.font_family(wrap.unsafe_cast_ref());
    if let Some(font_family) = ret {
        let font_family = font_family.to_glib_full();
        wrap.set_qdata(
            *FONT_CHOOSER_GET_FONT_FAMILY_QUARK,
            FontFamilyWrapper(font_family),
        );
        font_family
    } else {
        std::ptr::null_mut()
    }
}

static FONT_CHOOSER_GET_FONT_FACE_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk4-rs-subclass-font-chooser-font-face"));

#[derive(Debug)]
struct FontFaceWrapper(*mut pango::ffi::PangoFontFace);

impl Drop for FontFaceWrapper {
    fn drop(&mut self) {
        unsafe { glib::gobject_ffi::g_object_unref(self.0 as *mut _) }
    }
}

unsafe extern "C" fn font_chooser_get_font_face<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
) -> *mut pango::ffi::PangoFontFace {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.impl_();
    let wrap = from_glib_borrow::<_, FontChooser>(font_chooser);

    let ret = imp.font_face(wrap.unsafe_cast_ref());
    if let Some(font_face) = ret {
        let font_face = font_face.to_glib_full();
        wrap.set_qdata(
            *FONT_CHOOSER_GET_FONT_FACE_QUARK,
            FontFaceWrapper(font_face),
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
    let imp = instance.impl_();

    imp.font_size(from_glib_borrow::<_, FontChooser>(font_chooser).unsafe_cast_ref())
}

unsafe extern "C" fn font_chooser_font_activated<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
    font_nameptr: *const libc::c_char,
) {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.impl_();
    let font_name: Borrowed<GString> = from_glib_borrow(font_nameptr);

    imp.font_activated(
        from_glib_borrow::<_, FontChooser>(font_chooser).unsafe_cast_ref(),
        &font_name,
    )
}

unsafe extern "C" fn font_chooser_set_font_map<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
    font_mapptr: *mut pango::ffi::PangoFontMap,
) {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.impl_();
    let font_map: Borrowed<Option<FontMap>> = from_glib_borrow(font_mapptr);

    imp.set_font_map(
        from_glib_borrow::<_, FontChooser>(font_chooser).unsafe_cast_ref(),
        font_map.as_ref().as_ref(),
    )
}

unsafe extern "C" fn font_chooser_get_font_map<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
) -> *mut pango::ffi::PangoFontMap {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.impl_();

    imp.font_map(from_glib_borrow::<_, FontChooser>(font_chooser).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn font_chooser_set_filter_func<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
    filter_func: ffi::GtkFontFilterFunc,
    user_data: glib::ffi::gpointer,
    destroy_notify: glib::ffi::GDestroyNotify,
) {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.impl_();

    let callback = if filter_func.is_some() {
        None
    } else {
        Some(FilterCallback {
            filter_func,
            user_data,
            destroy_notify,
        })
    };

    imp.set_filter_func(
        from_glib_borrow::<_, FontChooser>(font_chooser).unsafe_cast_ref(),
        callback,
    );
}
