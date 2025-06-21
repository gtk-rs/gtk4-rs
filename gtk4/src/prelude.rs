// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for blanket imports.

#[doc(hidden)]
pub use gdk::prelude::*;
#[doc(hidden)]
pub use gsk::prelude::*;

pub use crate::{
    accessible::AccessibleExtManual,
    actionable::ActionableExtManual,
    auto::traits::*,
    cell_area::CellAreaExtManual,
    cell_layout::CellLayoutExtManual,
    color_chooser::ColorChooserExtManual,
    combo_box::ComboBoxExtManual,
    dialog::DialogExtManual,
    drawing_area::DrawingAreaExtManual,
    editable::EditableExtManual,
    entry::EntryExtManual,
    entry_buffer::EntryBufferExtManual,
    event_controller::EventControllerExtManual,
    expression::{GObjectPropertyExpressionExt, IsExpression},
    file_chooser::FileChooserExtManual,
    font_chooser::FontChooserExtManual,
    media_stream::MediaStreamExtManual,
    native_dialog::NativeDialogExtManual,
    scale::ScaleExtManual,
    shortcut_trigger::ShortcutTriggerExtManual,
    snapshot::SnapshotExtManual,
    text_buffer::TextBufferExtManual,
    tree_model::TreeModelExtManual,
    tree_model_filter::TreeModelFilterExtManual,
    tree_sortable::TreeSortableExtManual,
    tree_view::TreeViewExtManual,
    widget::WidgetExtManual,
};
