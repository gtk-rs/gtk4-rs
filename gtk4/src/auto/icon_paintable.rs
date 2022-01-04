// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "v4_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
use crate::SymbolicPaintable;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

#[cfg(any(feature = "v4_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
glib::wrapper! {
    #[doc(alias = "GtkIconPaintable")]
    pub struct IconPaintable(Object<ffi::GtkIconPaintable>) @implements gdk::Paintable, SymbolicPaintable;

    match fn {
        type_ => || ffi::gtk_icon_paintable_get_type(),
    }
}

#[cfg(not(any(feature = "v4_6", feature = "dox")))]
glib::wrapper! {
    #[doc(alias = "GtkIconPaintable")]
    pub struct IconPaintable(Object<ffi::GtkIconPaintable>) @implements gdk::Paintable;

    match fn {
        type_ => || ffi::gtk_icon_paintable_get_type(),
    }
}

impl IconPaintable {
    #[doc(alias = "gtk_icon_paintable_new_for_file")]
    #[doc(alias = "new_for_file")]
    pub fn for_file(file: &impl IsA<gio::File>, size: i32, scale: i32) -> IconPaintable {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_icon_paintable_new_for_file(
                file.as_ref().to_glib_none().0,
                size,
                scale,
            ))
        }
    }

    #[doc(alias = "gtk_icon_paintable_get_file")]
    #[doc(alias = "get_file")]
    pub fn file(&self) -> Option<gio::File> {
        unsafe { from_glib_full(ffi::gtk_icon_paintable_get_file(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_icon_paintable_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    pub fn icon_name(&self) -> Option<std::path::PathBuf> {
        unsafe { from_glib_none(ffi::gtk_icon_paintable_get_icon_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_icon_paintable_is_symbolic")]
    pub fn is_symbolic(&self) -> bool {
        unsafe { from_glib(ffi::gtk_icon_paintable_is_symbolic(self.to_glib_none().0)) }
    }
}

impl fmt::Display for IconPaintable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IconPaintable")
    }
}