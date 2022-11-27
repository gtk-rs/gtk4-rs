// Take a look at the license at the top of the repository in the LICENSE file.

use crate::FileChooser;
use glib::translate::*;
use glib::IsA;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`FileChooser`](crate::FileChooser).
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
pub trait FileChooserExtManual: 'static {
    #[doc(alias = "gtk_file_chooser_add_choice")]
    fn add_choice(&self, id: &str, label: &str, options: &[(&str, &str)]);

    #[doc(alias = "gtk_file_chooser_set_current_folder")]
    fn set_current_folder(&self, file: Option<&impl IsA<gio::File>>) -> Result<bool, glib::Error>;
}

impl<O: IsA<FileChooser>> FileChooserExtManual for O {
    fn add_choice(&self, id: &str, label: &str, options: &[(&str, &str)]) {
        unsafe {
            let stashes_ids = options
                .iter()
                .map(|o| o.0.to_glib_none())
                .collect::<Vec<_>>();
            let stashes_labels = options
                .iter()
                .map(|o| o.1.to_glib_none())
                .collect::<Vec<_>>();
            let (options_ids, options_labels) = (
                stashes_ids
                    .iter()
                    .map(|o| o.0)
                    .collect::<Vec<*const libc::c_char>>()
                    .as_ptr(),
                stashes_labels
                    .iter()
                    .map(|o| o.0)
                    .collect::<Vec<*const libc::c_char>>()
                    .as_ptr(),
            );

            ffi::gtk_file_chooser_add_choice(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                label.to_glib_none().0,
                mut_override(options_ids),
                mut_override(options_labels),
            );
        }
    }

    fn set_current_folder(&self, file: Option<&impl IsA<gio::File>>) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let result = from_glib(ffi::gtk_file_chooser_set_current_folder(
                self.as_ref().to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            ));
            if error.is_null() {
                Ok(result)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
