// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for creating custom types.

// rustdoc-stripper-ignore-next
/// Struct to hold a pointer and free it on `Drop::drop`
pub(crate) struct PtrHolder<T, F: Fn(*mut T) + 'static>(*mut T, F);

impl<T, F: Fn(*mut T) + 'static> Drop for PtrHolder<T, F> {
    fn drop(&mut self) {
        (self.1)(self.0)
    }
}

pub mod actionable;
pub mod adjustment;
pub mod application;
pub mod application_window;
pub mod box_;
pub mod buildable;
pub mod builder_scope;
pub mod button;
pub mod cell_area;
pub mod cell_area_context;
pub mod cell_editable;
pub mod cell_layout;
pub mod cell_renderer;
pub mod cell_renderer_text;
pub mod check_button;
pub mod color_chooser;
pub mod combo_box;
pub mod constraint_target;
pub mod dialog;
pub mod drawing_area;
pub mod editable;
pub mod entry;
pub mod entry_buffer;
pub mod filter;
pub mod fixed;
pub mod flow_box_child;
pub mod font_chooser;
pub mod frame;
pub mod gl_area;
pub mod grid;
pub mod im_context;
pub mod layout_child;
pub mod layout_manager;
pub mod list_box_row;
pub mod media_file;
pub mod media_stream;
pub mod native;
pub mod native_dialog;
pub mod orientable;
pub mod popover;
pub mod print_operation;
pub mod print_operation_preview;
pub mod range;
pub mod recent_manager;
pub mod root;
pub mod scale;
pub mod scale_button;
pub mod scrollable;
pub mod selection_model;
pub mod shortcut_manager;
pub mod sorter;
pub mod style_context;
#[cfg(any(feature = "v4_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
pub mod symbolic_paintable;
pub mod text_buffer;
pub mod text_view;
pub mod toggle_button;
pub mod tree_drag_dest;
pub mod tree_drag_source;
pub mod tree_model_filter;
pub mod tree_view;
pub mod widget;
pub mod window;

// rustdoc-stripper-ignore-next
/// Traits intended for blanket imports.
pub mod prelude {
    #[doc(hidden)]
    pub use gdk::subclass::prelude::*;
    #[doc(hidden)]
    pub use gio::subclass::prelude::*;
    #[doc(hidden)]
    pub use glib::subclass::prelude::*;

    pub use super::actionable::{ActionableImpl, ActionableImplExt};
    pub use super::adjustment::{AdjustmentImpl, AdjustmentImplExt};
    pub use super::application::{GtkApplicationImpl, GtkApplicationImplExt};
    pub use super::application_window::ApplicationWindowImpl;
    pub use super::box_::BoxImpl;
    pub use super::buildable::{BuildableImpl, BuildableImplExt};
    pub use super::builder_scope::{BuilderScopeImpl, BuilderScopeImplExt};
    pub use super::button::{ButtonImpl, ButtonImplExt};
    pub use super::cell_area::{CellAreaClassSubclassExt, CellAreaImpl, CellAreaImplExt};
    pub use super::cell_area_context::{CellAreaContextImpl, CellAreaContextImplExt};
    pub use super::cell_editable::{CellEditableImpl, CellEditableImplExt};
    pub use super::cell_layout::{CellLayoutImpl, CellLayoutImplExt};
    pub use super::cell_renderer::{CellRendererImpl, CellRendererImplExt};
    pub use super::cell_renderer_text::{CellRendererTextImpl, CellRendererTextImplExt};
    pub use super::check_button::{CheckButtonImpl, CheckButtonImplExt};
    pub use super::color_chooser::{ColorChooserImpl, ColorChooserImplExt};
    pub use super::combo_box::{ComboBoxImpl, ComboBoxImplExt};
    pub use super::constraint_target::ConstraintTargetImpl;
    pub use super::dialog::{DialogImpl, DialogImplExt};
    pub use super::drawing_area::{DrawingAreaImpl, DrawingAreaImplExt};
    pub use super::editable::{EditableImpl, EditableImplExt};
    pub use super::entry::{EntryImpl, EntryImplExt};
    pub use super::entry_buffer::{EntryBufferImpl, EntryBufferImplExt};
    pub use super::filter::{FilterImpl, FilterImplExt};
    pub use super::fixed::FixedImpl;
    pub use super::flow_box_child::{FlowBoxChildImpl, FlowBoxChildImplExt};
    pub use super::font_chooser::{FontChooserImpl, FontChooserImplExt};
    pub use super::frame::{FrameImpl, FrameImplExt};
    pub use super::gl_area::{GLAreaImpl, GLAreaImplExt};
    pub use super::grid::GridImpl;
    pub use super::im_context::{IMContextImpl, IMContextImplExt};
    pub use super::layout_child::LayoutChildImpl;
    pub use super::layout_manager::{LayoutManagerImpl, LayoutManagerImplExt};
    pub use super::list_box_row::{ListBoxRowImpl, ListBoxRowImplExt};
    pub use super::media_file::{MediaFileImpl, MediaFileImplExt};
    pub use super::media_stream::{MediaStreamImpl, MediaStreamImplExt};
    pub use super::native::NativeImpl;
    pub use super::native_dialog::{NativeDialogImpl, NativeDialogImplExt};
    pub use super::orientable::OrientableImpl;
    pub use super::popover::{PopoverImpl, PopoverImplExt};
    pub use super::print_operation::{PrintOperationImpl, PrintOperationImplExt};
    pub use super::print_operation_preview::PrintOperationPreviewImpl;
    pub use super::range::{RangeImpl, RangeImplExt};
    pub use super::recent_manager::{RecentManagerImpl, RecentManagerImplExt};
    pub use super::root::RootImpl;
    pub use super::scale::{ScaleImpl, ScaleImplExt};
    pub use super::scale_button::{ScaleButtonImpl, ScaleButtonImplExt};
    pub use super::scrollable::{ScrollableImpl, ScrollableImplExt};
    pub use super::selection_model::{SelectionModelImpl, SelectionModelImplExt};
    pub use super::shortcut_manager::{ShortcutManagerImpl, ShortcutManagerImplExt};
    pub use super::sorter::{SorterImpl, SorterImplExt};
    pub use super::style_context::{StyleContextImpl, StyleContextImplExt};
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub use super::symbolic_paintable::{SymbolicPaintableImpl, SymbolicPaintableImplExt};
    pub use super::text_buffer::{TextBufferImpl, TextBufferImplExt};
    pub use super::text_view::{TextViewImpl, TextViewImplExt};
    pub use super::toggle_button::{ToggleButtonImpl, ToggleButtonImplExt};
    pub use super::tree_drag_dest::{TreeDragDestImpl, TreeDragDestImplExt};
    pub use super::tree_drag_source::{TreeDragSourceImpl, TreeDragSourceImplExt};
    pub use super::tree_model_filter::{TreeModelFilterImpl, TreeModelFilterImplExt};
    pub use super::tree_view::{TreeViewImpl, TreeViewImplExt};
    pub use super::widget::CompositeTemplate;
    pub use super::widget::CompositeTemplateCallbacks;
    pub use super::widget::TemplateChild;
    pub use super::widget::WidgetClassSubclassExt;
    pub use super::widget::{WidgetImpl, WidgetImplExt};
    pub use super::window::{WindowImpl, WindowImplExt};
}
