// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[doc(alias = "gtk_accelerator_valid")]
pub fn accelerator_valid(keyval: gdk::keys::Key, modifiers: gdk::ModifierType) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_accelerator_valid(
            keyval.to_glib(),
            modifiers.to_glib(),
        ))
    }
}
