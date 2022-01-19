// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ShortcutTrigger;
use glib::translate::*;
use glib::IsA;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`ShortcutTrigger`](crate::ShortcutTrigger).
pub trait ShortcutTriggerExtManual {
    #[doc(alias = "gtk_shortcut_trigger_compare")]
    fn compare(&self, trigger2: &impl IsA<ShortcutTrigger>) -> std::cmp::Ordering;

    #[doc(alias = "gtk_shortcut_trigger_equal")]
    fn equal(&self, trigger2: &impl IsA<ShortcutTrigger>) -> bool;

    #[doc(alias = "gtk_shortcut_trigger_hash")]
    fn hash(&self) -> u32;

    #[doc(alias = "gtk_shortcut_trigger_trigger")]
    fn trigger(&self, event: &gdk::Event, enable_mnemonics: bool) -> gdk::KeyMatch;
}

impl<O: IsA<ShortcutTrigger>> ShortcutTriggerExtManual for O {
    fn compare(&self, trigger2: &impl IsA<ShortcutTrigger>) -> std::cmp::Ordering {
        unsafe {
            from_glib(ffi::gtk_shortcut_trigger_compare(
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(self.as_ref()).0
                    as glib::ffi::gconstpointer,
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(trigger2.as_ref()).0
                    as glib::ffi::gconstpointer,
            ))
        }
    }

    fn equal(&self, trigger2: &impl IsA<ShortcutTrigger>) -> bool {
        unsafe {
            from_glib(ffi::gtk_shortcut_trigger_equal(
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(self.as_ref()).0
                    as glib::ffi::gconstpointer,
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(trigger2.as_ref()).0
                    as glib::ffi::gconstpointer,
            ))
        }
    }

    fn hash(&self) -> u32 {
        unsafe {
            ffi::gtk_shortcut_trigger_hash(
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(self.as_ref()).0
                    as glib::ffi::gconstpointer,
            )
        }
    }

    fn trigger(&self, event: &gdk::Event, enable_mnemonics: bool) -> gdk::KeyMatch {
        unsafe {
            from_glib(ffi::gtk_shortcut_trigger_trigger(
                self.as_ref().to_glib_none().0,
                event.to_glib_none().0,
                enable_mnemonics.into_glib(),
            ))
        }
    }
}
