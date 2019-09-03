// Copyright 2013-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

// TODO: Introduction

#![cfg_attr(feature = "cargo-clippy", allow(let_unit_value))]
#![cfg_attr(feature = "cargo-clippy", allow(new_without_default))]
#![cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#![cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]
#![cfg_attr(feature = "cargo-clippy", allow(derive_hash_xor_eq))]
#![allow(deprecated)]

extern crate libc;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

extern crate atk_sys;
extern crate cairo_sys;
extern crate gdk4_sys as gdk_sys;
extern crate gdk_pixbuf_sys;
extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate graphene_sys;
extern crate gsk4_sys as gsk_sys;
extern crate gtk4_sys as gtk_sys;
extern crate pango_sys;
#[macro_use]
extern crate glib;
extern crate atk;
extern crate cairo;
extern crate gdk4 as gdk;
extern crate gdk_pixbuf;
extern crate gio;
extern crate graphene;
extern crate gsk4 as gsk;
extern crate pango;

#[cfg(feature = "futures")]
extern crate fragile;
#[cfg(feature = "futures")]
extern crate futures;

#[allow(unused_imports)]
use glib::{Cast, Continue, Error, IsA, Object, StaticType, ToValue, Type, TypedValue, Value};

pub const STYLE_PROVIDER_PRIORITY_FALLBACK: u32 =
    gtk_sys::GTK_STYLE_PROVIDER_PRIORITY_FALLBACK as u32;
pub const STYLE_PROVIDER_PRIORITY_THEME: u32 = gtk_sys::GTK_STYLE_PROVIDER_PRIORITY_THEME as u32;
pub const STYLE_PROVIDER_PRIORITY_SETTINGS: u32 =
    gtk_sys::GTK_STYLE_PROVIDER_PRIORITY_SETTINGS as u32;
pub const STYLE_PROVIDER_PRIORITY_APPLICATION: u32 =
    gtk_sys::GTK_STYLE_PROVIDER_PRIORITY_APPLICATION as u32;
pub const STYLE_PROVIDER_PRIORITY_USER: u32 = gtk_sys::GTK_STYLE_PROVIDER_PRIORITY_USER as u32;

#[macro_use]
mod rt;

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
#[cfg_attr(feature = "cargo-clippy", allow(let_and_return))]
#[cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]
#[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
#[allow(unused_imports)]
mod auto;

mod functions;
mod signal;

#[macro_use]
#[cfg(feature = "subclassing")]
pub mod subclass;

pub mod prelude;

pub use auto::functions::*;
pub use auto::*;
pub use functions::*;
pub use prelude::*;
pub use rt::*;

mod accel_key;
mod accel_map;
mod actionable;
mod application;
mod border;
mod builder;
mod color_chooser;
mod combo_box;
mod css_location;
mod dialog;
mod editable;
mod entry_buffer;
mod entry_completion;
mod file_chooser_dialog;
mod file_filter_info;
mod im_context_simple;
mod list_store;
mod message_dialog;
mod notebook;
mod overlay;
mod pad_action_entry;
mod pad_controller;
mod page_range;
mod recent_data;
mod requisition;
mod response_type;
mod shortcuts_section;
mod snapshot;
mod spin_button;
mod text;
mod text_buffer;
mod tree_model_filter;
mod tree_model_sort;
mod tree_path;
mod tree_sortable;
mod tree_store;
mod widget;

pub use accel_key::AccelKey;
pub use border::Border;
pub use css_location::CssLocation;
pub use file_filter_info::FileFilterInfo;
pub use pad_action_entry::PadActionEntry;
pub use page_range::PageRange;
pub use recent_data::RecentData;
pub use requisition::Requisition;
pub use response_type::ResponseType;
pub use tree_sortable::SortColumn;
pub use widget::TickCallbackId;
