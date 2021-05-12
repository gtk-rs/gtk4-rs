// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::{AboutDialog, Window};
use glib::translate::*;
use glib::{IsA, Quark, ToValue};
use once_cell::sync::Lazy;

#[doc(alias = "gtk_accelerator_valid")]
pub fn accelerator_valid(keyval: gdk::keys::Key, modifiers: gdk::ModifierType) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_accelerator_valid(
            keyval.into_glib(),
            modifiers.into_glib(),
        ))
    }
}

static SHOW_ABOUT_DIALOG_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-about-dialog"));

#[doc(alias = "gtk_show_about_dialog")]
pub fn show_about_dialog<P: IsA<Window>>(parent: Option<&P>, properties: &[(&str, &dyn ToValue)]) {
    assert_initialized_main_thread!();
    unsafe {
        if let Some(d) = parent.and_then(|p| p.qdata::<AboutDialog>(*SHOW_ABOUT_DIALOG_QUARK)) {
            d.as_ref().show();
        } else {
            let about_dialog = glib::Object::new::<AboutDialog>(properties)
                .expect("Failed to crate an about dialog");
            about_dialog.set_transient_for(parent);
            about_dialog.set_modal(true);
            about_dialog.set_destroy_with_parent(true);

            // cache the dialog if a parent is set
            if let Some(dialog_parent) = parent {
                dialog_parent.set_qdata(*SHOW_ABOUT_DIALOG_QUARK, about_dialog.clone());
            }
            about_dialog.show();
        };
    }
}
