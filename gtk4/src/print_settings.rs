// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{PageRange, PrintSettings};
use glib::translate::*;

impl PrintSettings {
    #[doc(alias = "gtk_print_settings_set_page_ranges")]
    pub fn set_page_ranges(&self, page_ranges: &[PageRange]) {
        let num_ranges = page_ranges.len() as i32;
        unsafe {
            ffi::gtk_print_settings_set_page_ranges(
                self.to_glib_none().0,
                mut_override(page_ranges.to_glib_none().0),
                num_ranges,
            );
        }
    }
}
