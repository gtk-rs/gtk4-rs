// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::wrong_self_convention)]
#![doc = include_str!("../README.md")]

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

#[doc(alias = "GTK_INVALID_LIST_POSITION")]
pub const INVALID_LIST_POSITION: u32 = ffi::GTK_INVALID_LIST_POSITION as u32;

#[doc(alias = "GTK_PRIORITY_RESIZE")]
pub const PRIORITY_RESIZE: u32 = ffi::GTK_PRIORITY_RESIZE as u32;
#[doc(alias = "GTK_TEXT_VIEW_PRIORITY_VALIDATE")]
pub const TEXT_VIEW_PRIORITY_VALIDATE: u32 = ffi::GTK_TEXT_VIEW_PRIORITY_VALIDATE as u32;

#[macro_use]
mod rt;

#[cfg(test)]
fn test_synced<F, R>(function: F) -> R
where
    F: FnOnce() -> R + Send + std::panic::UnwindSafe + 'static,
    R: Send + 'static,
{
    skip_assert_initialized!();

    use futures_channel::oneshot;
    use std::panic;

    let (tx, rx) = oneshot::channel();
    TEST_THREAD_WORKER
        .push(move || {
            tx.send(panic::catch_unwind(function))
                .unwrap_or_else(|_| panic!("Failed to return result from thread pool"));
        })
        .expect("Failed to schedule a test call");
    futures_executor::block_on(rx)
        .expect("Failed to receive result from thread pool")
        .unwrap_or_else(|e| std::panic::resume_unwind(e))
}

#[cfg(test)]
static TEST_THREAD_WORKER: once_cell::sync::Lazy<glib::ThreadPool> =
    once_cell::sync::Lazy::new(|| {
        let pool = glib::ThreadPool::exclusive(1).unwrap();
        pool.push(move || {
            crate::init().expect("Tests failed to initialize gtk");
        })
        .expect("Failed to schedule a test call");
        pool
    });

#[allow(clippy::let_and_return)]
#[allow(clippy::wrong_self_convention)]
#[allow(clippy::clone_on_copy)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::type_complexity)]
#[allow(unused_imports)]
mod auto;

#[macro_use]
pub mod subclass;
#[macro_use]
mod expression;

pub mod builders;
pub mod prelude;

pub use auto::functions::*;
pub use auto::*;
pub use rt::*;

mod accessible;
mod actionable;
mod application;
mod assistant;
mod bitset_iter;
mod bookmark_list;
mod border;
mod builder;
mod builder_rust_scope;
mod cell_area;
mod cell_editable;
mod cell_layout;
mod cell_renderer;
mod closure_expression;
mod color_chooser;
mod combo_box;
mod constant_expression;
mod constraint_guide;
mod constraint_layout;
mod css_location;
mod custom_filter;
mod custom_sorter;
mod dialog;
mod directory_list;
mod drawing_area;
mod drop_target;
mod editable;
mod entry;
mod entry_buffer;
mod entry_completion;
mod enums;
mod event_controller_key;
mod expression_watch;
mod file_chooser;
mod file_chooser_dialog;
mod flow_box;
mod font_chooser;
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
mod map_list_model;
mod media_stream;
mod menu_button;
mod message_dialog;
mod mnemonic_trigger;
mod native_dialog;
mod notebook;
mod object_expression;
mod overlay;
mod pad_action_entry;
mod pad_controller;
mod page_range;
mod param_spec_expression;
#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
mod print_job;
mod print_operation;
mod print_settings;
mod property_expression;
mod recent_data;
mod requisition;
mod response_type;
mod scale;
mod shortcut;
mod shortcut_trigger;
mod shortcuts_section;
mod snapshot;
mod spin_button;
mod string_list;
mod text;
mod text_buffer;
mod text_view;
mod tree_model;
mod tree_model_filter;
mod tree_path;
mod tree_row_reference;
mod tree_selection;
mod tree_sortable;
mod tree_store;
mod tree_view;
mod tree_view_column;
mod widget;

pub use bitset_iter::BitsetIter;
pub use border::Border;
pub use builder_rust_scope::BuilderRustScope;
pub use css_location::CssLocation;
pub use expression_watch::ExpressionWatch;
pub use functions::*;
pub use glib::signal::Inhibit;
pub use keyval_trigger::KeyvalTrigger;
pub use mnemonic_trigger::MnemonicTrigger;
pub use pad_action_entry::PadActionEntry;
pub use page_range::PageRange;
pub use param_spec_expression::ParamSpecExpression;
pub use recent_data::RecentData;
pub use response_type::ResponseType;
pub use tree_sortable::SortColumn;
pub use widget::TickCallbackId;
