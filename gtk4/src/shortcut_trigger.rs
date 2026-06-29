// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ShortcutTrigger, ffi, prelude::*};

impl ShortcutTrigger {
    #[cfg(feature = "v4_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_24")))]
    #[doc(alias = "gtk_shortcut_trigger_create_with_aliases")]
    pub fn create_with_aliases(key: gdk::Key, modifiers: gdk::ModifierType) -> ShortcutTrigger {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_shortcut_trigger_create_with_aliases(
                key.into_glib(),
                modifiers.into_glib(),
            ))
        }
    }
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of
/// [`ShortcutTrigger`](crate::ShortcutTrigger).
pub trait ShortcutTriggerExtManual: IsA<ShortcutTrigger> {
    #[doc(alias = "gtk_shortcut_trigger_compare")]
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

    #[doc(alias = "gtk_shortcut_trigger_equal")]
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

    #[doc(alias = "gtk_shortcut_trigger_hash")]
    fn hash(&self) -> u32 {
        unsafe {
            ffi::gtk_shortcut_trigger_hash(
                ToGlibPtr::<*mut ffi::GtkShortcutTrigger>::to_glib_none(self.as_ref()).0
                    as glib::ffi::gconstpointer,
            )
        }
    }
}

impl<O: IsA<ShortcutTrigger>> ShortcutTriggerExtManual for O {}
