// Take a look at the license at the top of the repository in the LICENSE file.

// TODO: Introduction

#![allow(clippy::let_unit_value)]
#![allow(clippy::new_without_default)]
#![allow(clippy::type_complexity)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::too_many_arguments)]
#![allow(deprecated)]

pub use ffi;
// Re-export gtk dependencies
pub use cairo;
pub use gdk;
pub use gdk_pixbuf;
pub use gio;
pub use glib;
pub use graphene;
pub use gsk;
pub use pango;

#[macro_use]
#[doc(hidden)]
#[allow(unused_imports)]
pub extern crate field_offset;
#[macro_use]
#[doc(hidden)]
#[allow(unused_imports)]
pub extern crate gtk4_macros;

#[doc(hidden)]
pub use field_offset::*;
#[doc(hidden)]
pub use gtk4_macros::*;

pub const STYLE_PROVIDER_PRIORITY_FALLBACK: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_FALLBACK as u32;
pub const STYLE_PROVIDER_PRIORITY_THEME: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_THEME as u32;
pub const STYLE_PROVIDER_PRIORITY_SETTINGS: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_SETTINGS as u32;
pub const STYLE_PROVIDER_PRIORITY_APPLICATION: u32 =
    ffi::GTK_STYLE_PROVIDER_PRIORITY_APPLICATION as u32;
pub const STYLE_PROVIDER_PRIORITY_USER: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_USER as u32;

#[macro_use]
mod rt;

#[allow(clippy::match_same_arms)]
#[allow(clippy::let_and_return)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::wrong_self_convention)]
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::clone_on_copy)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::cast_ptr_alignment)]
#[allow(unused_imports)]
mod auto;

mod signal;

#[macro_use]
pub mod subclass;

pub mod prelude;

pub use auto::functions::*;
pub use auto::*;
pub use rt::*;

mod actionable;
mod application;
mod border;
mod builder;
mod cell_area;
mod cell_editable;
mod cell_renderer;
mod color_chooser;
mod combo_box;
mod constraint_guide;
mod css_location;
mod custom_filter;
mod custom_sorter;
mod dialog;
mod drawing_area;
mod drop_target;
mod editable;
mod entry;
mod entry_buffer;
mod entry_completion;
mod enums;
mod event_controller_key;
mod expression;
mod file_chooser_dialog;
mod flow_box;
mod functions;
mod icon_theme;
mod im_context;
mod im_context_simple;
mod keyval_trigger;
mod label;
mod list_box;
mod list_store;
mod message_dialog;
mod mnemonic_trigger;
mod notebook;
mod overlay;
mod pad_action_entry;
mod pad_controller;
mod page_range;
mod recent_data;
mod requisition;
mod response_type;
mod shortcut_trigger;
mod shortcuts_section;
mod snapshot;
mod spin_button;
mod text;
mod text_buffer;
mod text_view;
mod tree_model_filter;
mod tree_model_sort;
mod tree_path;
mod tree_sortable;
mod tree_store;
mod widget;

pub use application::ApplicationBuilder;
pub use border::Border;
pub use css_location::CssLocation;
pub use expression::{
    ClosureExpression, ConstantExpression, Expression, ExpressionWatch, ObjectExpression,
    PropertyExpression, NONE_EXPRESSION,
};
pub use functions::*;
pub use pad_action_entry::PadActionEntry;
pub use page_range::PageRange;
pub use recent_data::RecentData;
pub use requisition::Requisition;
pub use response_type::ResponseType;
pub use tree_sortable::SortColumn;
pub use widget::TickCallbackId;
