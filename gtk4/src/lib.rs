// Take a look at the license at the top of the repository in the LICENSE file.

// TODO: Introduction

#![cfg_attr(feature = "dox", feature(doc_cfg))]
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
#[cfg(any(all(feature = "wayland", target_os = "linux"), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "wayland", target_os = "linux")))]
pub use gdk_wayland;
#[cfg(any(all(feature = "x11", target_os = "linux"), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "x11", target_os = "linux")))]
pub use gdk_x11;
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

#[doc(alias = "GTK_STYLE_PROVIDER_PRIORITY_FALLBACK")]
pub const STYLE_PROVIDER_PRIORITY_FALLBACK: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_FALLBACK as u32;
#[doc(alias = "GTK_STYLE_PROVIDER_PRIORITY_THEME")]
pub const STYLE_PROVIDER_PRIORITY_THEME: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_THEME as u32;
#[doc(alias = "GTK_STYLE_PROVIDER_PRIORITY_SETTINGS")]
pub const STYLE_PROVIDER_PRIORITY_SETTINGS: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_SETTINGS as u32;
#[doc(alias = "GTK_STYLE_PROVIDER_PRIORITY_APPLICATION")]
pub const STYLE_PROVIDER_PRIORITY_APPLICATION: u32 =
    ffi::GTK_STYLE_PROVIDER_PRIORITY_APPLICATION as u32;
#[doc(alias = "GTK_STYLE_PROVIDER_PRIORITY_USER")]
pub const STYLE_PROVIDER_PRIORITY_USER: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_USER as u32;

#[doc(alias = "GTK_ACCESSIBLE_VALUE_UNDEFINED")]
pub const ACCESSIBLE_VALUE_UNDEFINED: i32 = ffi::GTK_ACCESSIBLE_VALUE_UNDEFINED as i32;

#[doc(alias = "GTK_PRIORITY_RESIZE")]
pub const PRIORITY_RESIZE: u32 = ffi::GTK_PRIORITY_RESIZE as u32;
#[doc(alias = "GTK_TEXT_VIEW_PRIORITY_VALIDATE")]
pub const TEXT_VIEW_PRIORITY_VALIDATE: u32 = ffi::GTK_TEXT_VIEW_PRIORITY_VALIDATE as u32;

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
#[allow(clippy::upper_case_acronyms)]
#[allow(unused_imports)]
mod auto;

mod signal;

#[macro_use]
pub mod subclass;

pub mod prelude;

pub use auto::functions::*;
pub use auto::*;
pub use rt::*;

mod accessible;
mod actionable;
mod application;
mod bool_filter;
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
mod drop_down;
mod drop_target;
mod editable;
mod entry;
mod entry_buffer;
mod entry_completion;
mod enums;
mod event_controller_key;
mod expression;
mod file_chooser_dialog;
mod flags;
mod flow_box;
mod functions;
mod gesture_stylus;
mod icon_theme;
mod im_context;
mod im_context_simple;
mod info_bar;
mod keyval_trigger;
mod label;
mod list_box;
mod list_store;
mod media_stream;
mod message_dialog;
mod mnemonic_trigger;
mod notebook;
mod numeric_sorter;
mod overlay;
mod pad_action_entry;
mod pad_controller;
mod page_range;
#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
mod print_job;
mod recent_data;
mod requisition;
mod response_type;
mod shortcut_trigger;
mod shortcuts_section;
mod snapshot;
mod spin_button;
mod string_filter;
mod string_sorter;
mod text;
mod text_buffer;
mod text_view;
mod tree_model;
mod tree_model_filter;
mod tree_model_sort;
mod tree_path;
mod tree_row_reference;
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
#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
pub use flags::PrintCapabilities;
pub use functions::*;
pub use keyval_trigger::KeyvalTrigger;
pub use pad_action_entry::PadActionEntry;
pub use page_range::PageRange;
pub use recent_data::RecentData;
pub use requisition::Requisition;
pub use response_type::ResponseType;
pub use tree_sortable::SortColumn;
pub use widget::TickCallbackId;
