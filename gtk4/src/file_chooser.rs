// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, FileChooser};
use glib::{translate::*, IntoGStr};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`FileChooser`](crate::FileChooser).
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait FileChooserExtManual: 'static {
    #[doc(alias = "gtk_file_chooser_add_choice")]
    fn add_choice(&self, id: impl IntoGStr, label: impl IntoGStr, options: &[(&str, &str)]);

    #[doc(alias = "gtk_file_chooser_set_current_folder")]
    fn set_current_folder(&self, file: Option<&impl IsA<gio::File>>) -> Result<bool, glib::Error>;
}

impl<O: IsA<FileChooser>> FileChooserExtManual for O {
    fn add_choice(&self, id: impl IntoGStr, label: impl IntoGStr, options: &[(&str, &str)]) {
        unsafe {
            let (options_ids, options_labels) = if options.is_empty() {
                (std::ptr::null(), std::ptr::null())
            } else {
                let stashes_ids = options
                    .iter()
                    .map(|o| o.0.to_glib_none())
                    .collect::<Vec<_>>();
                let stashes_labels = options
                    .iter()
                    .map(|o| o.1.to_glib_none())
                    .collect::<Vec<_>>();
                (
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
                )
            };

            id.run_with_gstr(|id| {
                label.run_with_gstr(|label| {
                    ffi::gtk_file_chooser_add_choice(
                        self.as_ref().to_glib_none().0,
                        id.as_ptr(),
                        label.as_ptr(),
                        mut_override(options_ids),
                        mut_override(options_labels),
                    );
                });
            });
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
