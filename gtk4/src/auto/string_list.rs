// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Buildable;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkStringList")]
    pub struct StringList(Object<ffi::GtkStringList, ffi::GtkStringListClass>) @implements gio::ListModel, Buildable;

    match fn {
        type_ => || ffi::gtk_string_list_get_type(),
    }
}

impl StringList {
    #[doc(alias = "gtk_string_list_new")]
    pub fn new(strings: &[&str]) -> StringList {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_string_list_new(strings.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_string_list_append")]
    pub fn append(&self, string: &str) {
        unsafe {
            ffi::gtk_string_list_append(self.to_glib_none().0, string.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_string_list_get_string")]
    #[doc(alias = "get_string")]
    pub fn string(&self, position: u32) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_string_list_get_string(
                self.to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "gtk_string_list_remove")]
    pub fn remove(&self, position: u32) {
        unsafe {
            ffi::gtk_string_list_remove(self.to_glib_none().0, position);
        }
    }

    #[doc(alias = "gtk_string_list_splice")]
    pub fn splice(&self, position: u32, n_removals: u32, additions: &[&str]) {
        unsafe {
            ffi::gtk_string_list_splice(
                self.to_glib_none().0,
                position,
                n_removals,
                additions.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for StringList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StringList")
    }
}