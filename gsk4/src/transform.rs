use crate::Transform;
use glib::translate::*;
use std::fmt;

impl fmt::Display for Transform {
    #[doc(alias = "gsk_transform_print")]
    pub fn print(&self) -> Option<String> {
        unsafe {
            let mut string = String::uninitialized();
            let ret = ffi::gsk_transform_print(
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

