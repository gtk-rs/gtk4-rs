// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ComboBox;
use glib::object::IsA;
use glib::translate::*;

pub trait ComboBoxExtManual: 'static {
    #[doc(alias = "gtk_combo_box_set_active")]
    fn set_active(&self, index_: Option<u32>);

    #[doc(alias = "gtk_combo_box_get_active")]
    fn get_active(&self) -> Option<u32>;
}

impl<O: IsA<ComboBox>> ComboBoxExtManual for O {
    fn set_active(&self, index_: Option<u32>) {
        let index_ = match index_ {
            Some(i) => i as _,
            None => -1,
        };
        unsafe {
            ffi::gtk_combo_box_set_active(self.as_ref().to_glib_none().0, index_);
        }
    }

    fn get_active(&self) -> Option<u32> {
        match unsafe { ffi::gtk_combo_box_get_active(self.as_ref().to_glib_none().0) } {
            -1 => None,
            x => Some(x as _),
        }
    }
}
