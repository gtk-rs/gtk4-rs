use crate::ShortcutTrigger;
use glib::translate::*;
use glib::IsA;

pub trait ShortcutTriggerExtManual {
    fn compare<P: IsA<ShortcutTrigger>>(&self, trigger2: &P) -> i32;

    fn equal<P: IsA<ShortcutTrigger>>(&self, trigger2: &P) -> bool;

    fn hash(&self) -> u32;
}

impl ShortcutTriggerExtManual for ShortcutTrigger {
    fn compare<P: IsA<ShortcutTrigger>>(&self, trigger2: &P) -> i32 {
        unsafe {
            ffi::gtk_shortcut_trigger_compare(
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(self).0
                    as glib::ffi::gconstpointer,
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(trigger2.as_ref()).0
                    as glib::ffi::gconstpointer,
            )
        }
    }

    fn equal<P: IsA<ShortcutTrigger>>(&self, trigger2: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_shortcut_trigger_equal(
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(self).0
                    as glib::ffi::gconstpointer,
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(trigger2.as_ref()).0
                    as glib::ffi::gconstpointer,
            ))
        }
    }

    fn hash(&self) -> u32 {
        unsafe {
            ffi::gtk_shortcut_trigger_hash(
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(self).0
                    as glib::ffi::gconstpointer,
            )
        }
    }
}
