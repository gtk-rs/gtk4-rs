// Copyright 2015-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

//! Traits intended for blanket imports.

pub use crate::auto::traits::*;

pub use crate::actionable::ActionableExtManual;
pub use crate::builder::BuilderExtManual;
pub use crate::color_chooser::ColorChooserExtManual;
pub use crate::combo_box::ComboBoxExtManual;
pub use crate::dialog::DialogExtManual;
pub use crate::editable::EditableExtManual;
pub use crate::entry::EntryExtManual;
pub use crate::entry_buffer::EntryBufferExtManual;
pub use crate::entry_completion::EntryCompletionExtManual;
pub use crate::im_context_simple::IMContextSimpleExtManual;
pub use crate::list_store::GtkListStoreExtManual;
pub use crate::notebook::NotebookExtManual;
pub use crate::overlay::OverlayExtManual;
pub use crate::spin_button::SpinButtonExtManual;
pub use crate::text_buffer::TextBufferExtManual;
pub use crate::tree_sortable::TreeSortableExtManual;
pub use crate::tree_store::TreeStoreExtManual;
pub use crate::widget::WidgetExtManual;

#[doc(hidden)]
pub use glib::prelude::*;
