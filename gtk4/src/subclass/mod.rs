// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for creating custom types.

// rustdoc-stripper-ignore-next
/// Struct to hold a pointer and free it on `Drop::drop`
pub(crate) struct PtrHolder<T, F: Fn(*mut T) + 'static>(*mut T, F);

impl<T, F: Fn(*mut T) + 'static> Drop for PtrHolder<T, F> {
    #[inline]
    fn drop(&mut self) {
        (self.1)(self.0)
    }
}

#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
pub mod accessible;
#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
pub mod accessible_range;
#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
pub mod accessible_text;
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
pub mod native_dialog;
pub mod orientable;
pub mod popover;
pub mod print_operation;
pub mod print_operation_preview;
pub mod range;
pub mod recent_manager;
pub mod scale;
pub mod scale_button;
pub mod scrollable;
#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
pub mod section_model;
pub mod selection_model;
pub mod shortcut_manager;
pub mod sorter;
pub mod style_context;
#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
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
pub mod window_group;

// rustdoc-stripper-ignore-next
/// Traits intended for blanket imports.
pub mod prelude {
    #[doc(hidden)]
    pub use gdk::subclass::prelude::*;

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub use super::accessible::{AccessibleImpl, AccessibleImplExt};
    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub use super::accessible_range::{AccessibleRangeImpl, AccessibleRangeImplExt};
    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    pub use super::accessible_text::{AccessibleTextImpl, AccessibleTextImplExt};
    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    pub use super::section_model::{SectionModelImpl, SectionModelImplExt};
    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    pub use super::symbolic_paintable::{SymbolicPaintableImpl, SymbolicPaintableImplExt};
    pub use super::{
        actionable::{ActionableImpl, ActionableImplExt},
        adjustment::{AdjustmentImpl, AdjustmentImplExt},
        application::{GtkApplicationImpl, GtkApplicationImplExt},
        application_window::ApplicationWindowImpl,
        box_::BoxImpl,
        buildable::{BuildableImpl, BuildableImplExt},
        builder_scope::{BuilderCScopeImpl, BuilderScopeImpl, BuilderScopeImplExt},
        button::{ButtonImpl, ButtonImplExt},
        cell_area::{CellAreaClassExt, CellAreaImpl, CellAreaImplExt},
        cell_area_context::{CellAreaContextImpl, CellAreaContextImplExt},
        cell_editable::{CellEditableImpl, CellEditableImplExt},
        cell_layout::{CellLayoutImpl, CellLayoutImplExt},
        cell_renderer::{CellRendererImpl, CellRendererImplExt},
        cell_renderer_text::{CellRendererTextImpl, CellRendererTextImplExt},
        check_button::{CheckButtonImpl, CheckButtonImplExt},
        color_chooser::{ColorChooserImpl, ColorChooserImplExt},
        combo_box::{ComboBoxImpl, ComboBoxImplExt},
        dialog::{DialogImpl, DialogImplExt},
        drawing_area::{DrawingAreaImpl, DrawingAreaImplExt},
        editable::{EditableImpl, EditableImplExt},
        entry::{EntryImpl, EntryImplExt},
        entry_buffer::{EntryBufferImpl, EntryBufferImplExt},
        filter::{FilterImpl, FilterImplExt},
        fixed::FixedImpl,
        flow_box_child::{FlowBoxChildImpl, FlowBoxChildImplExt},
        font_chooser::{FontChooserImpl, FontChooserImplExt},
        frame::{FrameImpl, FrameImplExt},
        gl_area::{GLAreaImpl, GLAreaImplExt},
        grid::GridImpl,
        im_context::{IMContextImpl, IMContextImplExt},
        layout_child::LayoutChildImpl,
        layout_manager::{LayoutManagerImpl, LayoutManagerImplExt},
        list_box_row::{ListBoxRowImpl, ListBoxRowImplExt},
        media_file::{MediaFileImpl, MediaFileImplExt},
        media_stream::{MediaStreamImpl, MediaStreamImplExt},
        native_dialog::{NativeDialogImpl, NativeDialogImplExt},
        orientable::OrientableImpl,
        popover::{PopoverImpl, PopoverImplExt},
        print_operation::{PrintOperationImpl, PrintOperationImplExt},
        print_operation_preview::PrintOperationPreviewImpl,
        range::{RangeImpl, RangeImplExt},
        recent_manager::{RecentManagerImpl, RecentManagerImplExt},
        scale::{ScaleImpl, ScaleImplExt},
        scale_button::{ScaleButtonImpl, ScaleButtonImplExt},
        scrollable::{ScrollableImpl, ScrollableImplExt},
        selection_model::{SelectionModelImpl, SelectionModelImplExt},
        shortcut_manager::{ShortcutManagerImpl, ShortcutManagerImplExt},
        sorter::{SorterImpl, SorterImplExt},
        style_context::{StyleContextImpl, StyleContextImplExt},
        text_buffer::{TextBufferImpl, TextBufferImplExt},
        text_view::{TextViewImpl, TextViewImplExt},
        toggle_button::{ToggleButtonImpl, ToggleButtonImplExt},
        tree_drag_dest::{TreeDragDestImpl, TreeDragDestImplExt},
        tree_drag_source::{TreeDragSourceImpl, TreeDragSourceImplExt},
        tree_model_filter::{TreeModelFilterImpl, TreeModelFilterImplExt},
        tree_view::{TreeViewImpl, TreeViewImplExt},
        widget::{
            CompositeTemplate, CompositeTemplateCallbacks, CompositeTemplateCallbacksClass,
            CompositeTemplateClass, CompositeTemplateDisposeExt, CompositeTemplateInitializingExt,
            CompositeTemplateInstanceCallbacksClass, TemplateChild, WidgetClassExt, WidgetImpl,
            WidgetImplExt,
        },
        window::{WindowImpl, WindowImplExt},
        window_group::WindowGroupImpl,
    };
}
