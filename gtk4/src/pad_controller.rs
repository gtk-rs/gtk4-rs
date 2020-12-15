// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{PadActionEntry, PadController};
use glib::translate::*;

impl PadController {
    pub fn set_action_entries(&self, entries: &[PadActionEntry]) {
        let n_entries = entries.len() as i32;
        let entry_strings = entries
            .iter()
            .map(|e| {
                (
                    e.get_label().to_glib_none(),
                    e.get_action_name().to_glib_none(),
                )
            })
            .collect::<Vec<_>>();
        let entries = entries
            .iter()
            .zip(entry_strings.iter())
            .map(|(e, (label, action_name))| ffi::GtkPadActionEntry {
                type_: e.get_type().to_glib(),
                index: e.get_index(),
                mode: e.get_mode(),
                label: label.0,
                action_name: action_name.0,
            })
            .collect::<Vec<_>>();
        unsafe {
            ffi::gtk_pad_controller_set_action_entries(
                self.to_glib_none().0,
                entries.as_ptr(),
                n_entries,
            );
        }
    }
}
