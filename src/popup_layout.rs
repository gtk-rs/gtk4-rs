use auto::PopupLayout;
use glib::translate::*;

pub trait PopupLayoutExtManual {
    fn set_offset(&self, dx: i32, dy: i32);
}

impl PopupLayoutExtManual for PopupLayout {
    fn set_offset(&self, dx: i32, dy: i32) {
        unsafe {
            gdk_sys::gdk_popup_layout_set_offset(self.to_glib_none().0, dx, dy);
        }
    }
}
