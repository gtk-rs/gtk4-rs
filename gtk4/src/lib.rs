// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! # Rust GTK 4 bindings
//!
//! This library contains safe Rust bindings for [GTK 4](http://www.gtk.org), a
//! multi-platform GUI toolkit. It is a part of [gtk-rs](http://gtk-rs.org/).
//!
//! Most of this documentation is generated from the C API.
//! Until all parts of the documentation have been reviewed there will be incongruities
//! with the actual Rust API.
//!
//! For a gentle introduction to gtk-rs we recommend the online book
//! [*GUI development with Rust and GTK 4*](../../book/).
//!
//! See also
//!
//! - [gtk-rs project overview](https://gtk-rs.org)
//!
//! - [General `GLib` family types and object system overview](mod@glib)
//!
//! - [GTK documentation](https://www.gtk.org/docs/)
//!
//! # "Hello, World!" example program
//!
//! GTK needs to be initialized before use by calling [`fn@init`]. Creating an
//! [`struct@Application`] will call [`fn@init`] for you.
//!
//! The [`gtk4`](mod@crate) crate is usually renamed to `gtk`. You can find an example in
//! the [features](#features) section for how to do this globally in your `Cargo.toml`.
//!
//! ```no_run
//! # use gtk4 as gtk;
//! use gtk::prelude::*;
//! use gtk::{Application, ApplicationWindow};
//!
//! fn main() {
//!     let app = Application::builder()
//!         .application_id("org.example.HelloWorld")
//!         .build();
//!
//!     app.connect_activate(|app| {
//!         // We create the main window.
//!         let window = ApplicationWindow::builder()
//!             .application(app)
//!             .default_width(320)
//!             .default_height(200)
//!             .title("Hello, World!")
//!             .build();
//!
//!         // Show the window.
//!         window.show();
//!     });
//!
//!     app.run();
//! }
//! ```
//!
//! # The main loop
//!
//! In a typical GTK application you set up the UI, assign signal handlers
//! and run the main event loop.
//!
//! ```no_run
//! # use gtk4 as gtk;
//! use gtk::prelude::*;
//! use gtk::{Application, ApplicationWindow, Button};
//!
//! fn main() {
//!     let application = Application::builder()
//!         .application_id("com.example.FirstGtkApp")
//!         .build();
//!
//!     application.connect_activate(|app| {
//!         let window = ApplicationWindow::builder()
//!             .application(app)
//!             .title("First GTK Program")
//!             .default_width(350)
//!             .default_height(70)
//!             .build();
//!
//!         let button = Button::with_label("Click me!");
//!         button.connect_clicked(|_| {
//!             eprintln!("Clicked!");
//!         });
//!         window.set_child(Some(&button));
//!
//!         window.show();
//!     });
//!
//!     application.run();
//! }
//! ```
//!
//! # Threads
//!
//! GTK is not thread-safe. Accordingly, none of this crate's structs implement
//! [`Send`] or [`Sync`].
//!
//! The thread where [`fn@init`] was called is considered the main thread. OS X has
//! its own notion of the main thread and [`fn@init`] must be called on that thread.
//! After successful initialization, calling any [`gtk`](mod@crate) or [`mod@gdk`] functions
//! (including [`fn@init`]) from other threads will `panic`.
//!
//! Any thread can schedule a closure to be run by the main loop on the main
//! thread via [`fn@glib::idle_add`] or [`fn@glib::timeout_add`]. While
//! working with GTK you might need the [`fn@glib::idle_add_local`]
//! or [`fn@glib::timeout_add_local`] version without the
//! [`Send`] bound. Those may only be called from the main thread.
//!
//! # Panics
//!
//! The [`gtk`](mod@crate) and [`mod@gdk`] crates have some run-time safety and contract checks.
//!
//! - Any constructor or free function will panic if called before [`fn@init`] or on
//! a non-main thread.
//!
//! - Any [`&str`] or [`&Path`](std::path::Path) parameter with an interior null (`\0`) character will
//! cause a panic.
//!
//! - Some functions will panic if supplied out-of-range integer parameters. All
//! such cases will be documented individually but they are not yet.
//!
//! - A panic in a closure that handles signals or in any other closure passed
//! to a [`gtk`](mod@crate) function will abort the process.
//!
//! # Features
//!
//! ## Library versions
//!
//! By default this crate provides only GTK 4.0 APIs. You can access additional
//! functionality by selecting one of the `v4_2`, `v4_4`, etc. features.
//!
//! `Cargo.toml` example:
//!
//! ```toml
//! [dependencies.gtk]
//! package = "gtk4"
//! version = "0.x.y"
//! features = ["v4_2"]
//! ```
//!
//! Take care when choosing the version to target: some of your users might
//! not have easy access to the latest ones. The higher the version, the fewer
//! users will have it installed.
#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::wrong_self_convention)]

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

#[allow(dead_code)]
#[cfg(test)]
pub(crate) static TEST_THREAD_WORKER: once_cell::sync::Lazy<glib::ThreadPool> =
    once_cell::sync::Lazy::new(|| {
        let pool = glib::ThreadPool::new_exclusive(1).unwrap();
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
mod bool_filter;
mod border;
mod builder;
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
mod drop_down;
mod drop_target;
mod editable;
mod entry;
mod entry_buffer;
mod entry_completion;
mod enums;
mod event_controller_key;
mod expression_watch;
mod file_chooser_dialog;
mod flow_box;
mod font_chooser;
mod functions;
mod gesture_stylus;
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
mod numeric_sorter;
mod object_expression;
mod overlay;
mod pad_action_entry;
mod pad_controller;
mod page_range;
mod param_spec_expression;
#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
mod print_job;
mod property_expression;
mod recent_data;
mod requisition;
mod response_type;
mod scale;
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
mod tree_path;
mod tree_row_reference;
mod tree_selection;
mod tree_sortable;
mod tree_store;
mod tree_view;
mod tree_view_column;
mod widget;

pub use bitset_iter::BitsetIter;
pub use bookmark_list::BookmarkListBuilder;
pub use border::{Border, BorderBuilder};
pub use closure_expression::ClosureExpression;
pub use constant_expression::ConstantExpression;
pub use css_location::CssLocation;
pub use directory_list::DirectoryListBuilder;
pub use expression::{Expression, NONE_EXPRESSION};
pub use expression_watch::ExpressionWatch;
pub use functions::*;
pub use glib::signal::Inhibit;
pub use keyval_trigger::KeyvalTrigger;
pub use mnemonic_trigger::MnemonicTrigger;
pub use object_expression::ObjectExpression;
pub use pad_action_entry::PadActionEntry;
pub use page_range::PageRange;
pub use param_spec_expression::ParamSpecExpression;
pub use property_expression::PropertyExpression;
pub use recent_data::RecentData;
pub use requisition::Requisition;
pub use response_type::ResponseType;
pub use tree_sortable::SortColumn;
pub use widget::TickCallbackId;
