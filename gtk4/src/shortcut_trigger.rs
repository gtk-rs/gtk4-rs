use glib::translate::*;
use glib::IsA;
use ShortcutTrigger;

pub trait ShortcutTriggerExtManual {
    fn compare<P: IsA<ShortcutTrigger>>(&self, trigger2: &P) -> i32;

    fn equal<P: IsA<ShortcutTrigger>>(&self, trigger2: &P) -> bool;

    fn hash(&self) -> u32;
}

impl ShortcutTriggerExtManual for ShortcutTrigger {
    fn compare<P: IsA<ShortcutTrigger>>(&self, trigger2: &P) -> i32 {
        unsafe {
            gtk_sys::gtk_shortcut_trigger_compare(
                ToGlibPtr::<*mut gtk_sys::GtkShortcutTrigger>::to_glib_none(self).0
                    as glib_sys::gconstpointer,
                ToGlibPtr::<*mut gtk_sys::GtkShortcutTrigger>::to_glib_none(trigger2.as_ref()).0
                    as glib_sys::gconstpointer,
            )
        }
    }

    fn equal<P: IsA<ShortcutTrigger>>(&self, trigger2: &P) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_shortcut_trigger_equal(
                ToGlibPtr::<*mut gtk_sys::GtkShortcutTrigger>::to_glib_none(self).0
                    as glib_sys::gconstpointer,
                ToGlibPtr::<*mut gtk_sys::GtkShortcutTrigger>::to_glib_none(trigger2.as_ref()).0
                    as glib_sys::gconstpointer,
            ))
        }
    }

    fn hash(&self) -> u32 {
        unsafe {
            gtk_sys::gtk_shortcut_trigger_hash(
                ToGlibPtr::<*mut gtk_sys::GtkShortcutTrigger>::to_glib_none(self).0
                    as glib_sys::gconstpointer,
            )
        }
    }
}
