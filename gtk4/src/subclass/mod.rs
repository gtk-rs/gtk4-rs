// Take a look at the license at the top of the repository in the LICENSE file.

pub mod actionable;
pub mod adjustment;
pub mod application;
pub mod application_window;
pub mod box_;
pub mod button;
pub mod cell_renderer;
pub mod cell_renderer_text;
pub mod check_button;
pub mod dialog;
pub mod drawing_area;
pub mod editable;
pub mod entry;
pub mod filter;
pub mod flow_box_child;
pub mod frame;
pub mod gl_area;
pub mod layout_manager;
pub mod list_box_row;
pub mod media_file;
pub mod media_stream;
pub mod native_dialog;
pub mod orientable;
pub mod popover;
pub mod recent_manager;
pub mod scrollable;
pub mod shortcut_manager;
pub mod sorter;
pub mod style_context;
pub mod text_buffer;
pub mod text_view;
pub mod toggle_button;
pub mod widget;
pub mod window;

pub mod prelude {
    #[doc(hidden)]
    pub use gdk::subclass::prelude::*;
    #[doc(hidden)]
    pub use gio::subclass::prelude::*;
    #[doc(hidden)]
    pub use glib::subclass::prelude::*;

    pub use super::actionable::ActionableImpl;
    pub use super::adjustment::AdjustmentImpl;
    pub use super::application::GtkApplicationImpl;
    pub use super::application_window::ApplicationWindowImpl;
    pub use super::box_::BoxImpl;
    pub use super::button::ButtonImpl;
    pub use super::cell_renderer::CellRendererImpl;
    pub use super::cell_renderer_text::CellRendererTextImpl;
    pub use super::check_button::CheckButtonImpl;
    pub use super::dialog::DialogImpl;
    pub use super::drawing_area::DrawingAreaImpl;
    pub use super::editable::EditableImpl;
    pub use super::entry::EntryImpl;
    pub use super::filter::FilterImpl;
    pub use super::flow_box_child::FlowBoxChildImpl;
    pub use super::frame::FrameImpl;
    pub use super::gl_area::GLAreaImpl;
    pub use super::layout_manager::LayoutManagerImpl;
    pub use super::list_box_row::ListBoxRowImpl;
    pub use super::media_file::MediaFileImpl;
    pub use super::media_stream::MediaStreamImpl;
    pub use super::native_dialog::NativeDialogImpl;
    pub use super::orientable::OrientableImpl;
    pub use super::popover::PopoverImpl;
    pub use super::recent_manager::RecentManagerImpl;
    pub use super::scrollable::ScrollableImpl;
    pub use super::shortcut_manager::ShortcutManagerImpl;
    pub use super::sorter::SorterImpl;
    pub use super::style_context::StyleContextImpl;
    pub use super::text_buffer::TextBufferImpl;
    pub use super::text_view::TextViewImpl;
    pub use super::toggle_button::ToggleButtonImpl;
    pub use super::widget::CompositeTemplate;
    pub use super::widget::TemplateChild;
    pub use super::widget::WidgetClassSubclassExt;
    pub use super::widget::WidgetImpl;
    pub use super::window::WindowImpl;
}
