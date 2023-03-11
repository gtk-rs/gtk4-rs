// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub use cairo;
pub use ffi;
pub use gdk_pixbuf;
pub use gio;
pub use glib;
pub use pango;

#[doc(alias = "GDK_PRIORITY_REDRAW")]
pub const PRIORITY_REDRAW: u32 = ffi::GDK_PRIORITY_REDRAW as u32;

#[doc(alias = "GDK_MODIFIER_MASK")]
pub const MODIFIER_MASK: ModifierType = ModifierType::all();

#[doc(alias = "GDK_ACTION_ALL")]
pub const ACTION_ALL: u32 = ffi::GDK_ACTION_ALL as u32;

#[doc(alias = "GDK_CURRENT_TIME")]
pub const CURRENT_TIME: u32 = ffi::GDK_CURRENT_TIME as u32;

#[doc(alias = "GDK_BUTTON_PRIMARY")]
pub const BUTTON_PRIMARY: u32 = ffi::GDK_BUTTON_PRIMARY as u32;

#[doc(alias = "GDK_BUTTON_MIDDLE")]
pub const BUTTON_MIDDLE: u32 = ffi::GDK_BUTTON_MIDDLE as u32;

#[doc(alias = "GDK_BUTTON_SECONDARY")]
pub const BUTTON_SECONDARY: u32 = ffi::GDK_BUTTON_SECONDARY as u32;

#[doc(alias = "GDK_EVENT_STOP")]
pub const EVENT_STOP: u32 = ffi::GDK_EVENT_STOP as u32;

#[doc(alias = "GDK_EVENT_PROPAGATE")]
pub const EVENT_PROPAGATE: u32 = ffi::GDK_EVENT_PROPAGATE as u32;

// GDK 4 has no runtime to initialize
macro_rules! assert_initialized_main_thread {
    () => {};
}

// No-op
macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::derived_hash_with_manual_eq)]
#[allow(clippy::type_complexity)]
mod auto;

#[macro_use]
mod event;

pub mod builders;
pub mod prelude;
pub mod subclass;

mod button_event;
mod cairo_interaction;
mod clipboard;
mod content_deserializer;
mod content_formats;
mod content_formats_builder;
mod content_provider;
mod content_serializer;
mod crossing_event;
mod delete_event;
mod display;
mod dnd_event;
mod draw_context;
mod drop;
mod focus_event;
mod functions;
mod gl_texture;
mod grab_broken_event;
mod key_event;
mod keymap_key;
mod keys;
mod motion_event;
mod pad_event;
mod popup_layout;
mod proximity_event;
mod rectangle;
mod rgba;
mod scroll_event;
mod surface;
mod texture;
mod time_coord;
mod toplevel;
mod toplevel_size;
mod touch_event;
mod touchpad_event;

pub use self::auto::functions::*;
pub use auto::*;

pub use functions::*;

pub use display::Backend;
pub use keymap_key::KeymapKey;
pub use keys::Key;
pub use time_coord::TimeCoord;
pub use toplevel_size::ToplevelSize;
