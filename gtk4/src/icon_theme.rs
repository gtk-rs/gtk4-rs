use crate::ffi;
use crate::IconTheme;
use glib::translate::*;

pub trait IconThemeExtManual {
    fn set_resource_path(&self, path: &str);
}

impl<O: AsRef<IconTheme>> IconThemeExtManual for O {
    fn set_resource_path(&self, path: &str) {
        unsafe {
            ffi::gtk_icon_theme_set_resource_path(
                self.as_ref().to_glib_none().0,
                path.as_ptr() as *const _,
            );
        }
    }
}
