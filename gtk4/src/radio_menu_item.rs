use glib::object::Cast;
use glib::translate::*;
use std::ptr;
use gtk_sys;
use RadioMenuItem;
use Widget;

impl RadioMenuItem {
    pub fn new() -> RadioMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_radio_menu_item_new(ptr::null_mut())).unsafe_cast()
        }
    }

    pub fn new_with_label(label: &str) -> RadioMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_radio_menu_item_new_with_label(ptr::null_mut(), label.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> RadioMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_radio_menu_item_new_with_mnemonic(ptr::null_mut(), label.to_glib_none().0)).unsafe_cast()
        }
    }
}
