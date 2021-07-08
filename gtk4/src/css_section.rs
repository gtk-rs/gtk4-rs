use crate CssSection;
use glib::translate::*;
use std::fmt;

impl fmt::Display for CssSection {
    #[doc(alias = "gtk_css_section_print")]
    pub fn print(&self) -> Option<String> {
        unsafe {
            let mut string = String::uninitialized();
            let ret = ffi::gtk_css_section_print(
                self.to_glib_none().0,
                 string.to_glib_none_mut().0,
                );
            if from_glib(ret) {
                Some(string)
            } else {
                None
            }
        }
    }
}