use gdk;
use gdk_sys;
use glib::GString;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use libc;
use AccelMap;

impl AccelMap {
    pub fn foreach<P: FnMut(&str, u32, &gdk::ModifierType, bool)>(foreach_func: P) {
        assert_initialized_main_thread!();
        let foreach_func_data: P = foreach_func;
        unsafe extern "C" fn foreach_func_func<P: FnMut(&str, u32, &gdk::ModifierType, bool)>(data: glib_sys::gpointer, accel_path: *const libc::c_char, accel_key: libc::c_uint, accel_mods: gdk_sys::GdkModifierType, changed: glib_sys::gboolean) {
            let accel_path: GString = from_glib_borrow(accel_path);
            let accel_mods = from_glib(accel_mods);
            let changed = from_glib(changed);
            let callback: *mut P = data as *const _ as usize as *mut P;
            (*callback)(accel_path.as_str(), accel_key, &accel_mods, changed);
        }
        let foreach_func = Some(foreach_func_func::<P> as _);
        let super_callback0: &P = &foreach_func_data;
        unsafe {
            gtk_sys::gtk_accel_map_foreach(super_callback0 as *const _ as usize as *mut _, foreach_func);
        }
    }

    pub fn foreach_unfiltered<P: FnMut(&str, u32, &gdk::ModifierType, bool)>(foreach_func: P) {
        assert_initialized_main_thread!();
        let foreach_func_data: P = foreach_func;
        unsafe extern "C" fn foreach_func_func<P: FnMut(&str, u32, &gdk::ModifierType, bool)>(data: glib_sys::gpointer, accel_path: *const libc::c_char, accel_key: libc::c_uint, accel_mods: gdk_sys::GdkModifierType, changed: glib_sys::gboolean) {
            let accel_path: GString = from_glib_borrow(accel_path);
            let accel_mods = from_glib(accel_mods);
            let changed = from_glib(changed);
            let callback: *mut P = data as *const _ as usize as *mut P;
            (*callback)(accel_path.as_str(), accel_key, &accel_mods, changed);
        }
        let foreach_func = Some(foreach_func_func::<P> as _);
        let super_callback0: &P = &foreach_func_data;
        unsafe {
            gtk_sys::gtk_accel_map_foreach_unfiltered(super_callback0 as *const _ as usize as *mut _, foreach_func);
        }
    }
}
