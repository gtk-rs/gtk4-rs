// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::{AboutDialog, Window};
use glib::translate::*;
use glib::{IsA, ToValue};

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

#[doc(alias = "gtk_show_about_dialog")]
pub fn show_about_dialog<P: IsA<Window>>(parent: Option<&P>, properties: &[(&str, &dyn ToValue)]) {
    assert_initialized_main_thread!();

    let about_dialog =
        glib::Object::new::<AboutDialog>(properties).expect("Failed to crate an about dialog");
    about_dialog.set_transient_for(parent);
    about_dialog.show();
}

#[doc(alias = "gtk_test_list_all_types")]
pub fn test_list_all_types() -> Vec<glib::Type> {
    unsafe {
        let mut n_types = std::mem::MaybeUninit::uninit();
        let types = ffi::gtk_test_list_all_types(n_types.as_mut_ptr());
        FromGlibContainer::from_glib_container_num(types, n_types.assume_init() as usize)
    }
}
