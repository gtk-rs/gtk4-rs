// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ShortcutTrigger;
use glib::translate::*;
use glib::IsA;

impl ShortcutTrigger {
    #[doc(alias = "gtk_shortcut_trigger_compare")]
    pub fn compare<P: IsA<ShortcutTrigger>>(&self, trigger2: &P) -> std::cmp::Ordering {
        unsafe {
            from_glib(ffi::gtk_shortcut_trigger_compare(
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(self).0
                    as glib::ffi::gconstpointer,
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(trigger2.as_ref()).0
                    as glib::ffi::gconstpointer,
            ))
        }
    }

    #[doc(alias = "gtk_shortcut_trigger_equal")]
    pub fn equal<P: IsA<ShortcutTrigger>>(&self, trigger2: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_shortcut_trigger_equal(
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(self).0
                    as glib::ffi::gconstpointer,
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(trigger2.as_ref()).0
                    as glib::ffi::gconstpointer,
            ))
        }
    }

    #[doc(alias = "gtk_shortcut_trigger_hash")]
    pub fn hash(&self) -> u32 {
        unsafe {
            ffi::gtk_shortcut_trigger_hash(
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(self).0
                    as glib::ffi::gconstpointer,
            )
        }
    }

    #[doc(alias = "gtk_shortcut_trigger_trigger")]
    pub fn trigger(&self, event: &gdk::Event, enable_mnemonics: bool) -> gdk::KeyMatch {
        unsafe {
            from_glib(ffi::gtk_shortcut_trigger_trigger(
                self.to_glib_none().0,
                event.to_glib_none().0,
                enable_mnemonics.into_glib(),
            ))
        }
    }
}
