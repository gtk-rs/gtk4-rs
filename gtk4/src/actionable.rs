// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Actionable;
use glib::object::IsA;
use glib::translate::*;
use libc::c_char;

pub trait ActionableExtManual: 'static {
    #[doc(alias = "gtk_actionable_set_action_target")]
    fn set_action_target(&self, string: &str);
}

impl<O: IsA<Actionable>> ActionableExtManual for O {
    fn set_action_target(&self, string: &str) {
        let string: Stash<*const c_char, _> = string.to_glib_none();
        unsafe {
            ffi::gtk_actionable_set_action_target(
                self.as_ref().to_glib_none().0,
                b"%s\0".as_ptr() as *const c_char,
                string.0,
            );
        }
    }
}
