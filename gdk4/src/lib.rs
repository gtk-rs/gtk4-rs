// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::let_unit_value)]
#![allow(clippy::new_without_default)]
#![allow(clippy::type_complexity)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::too_many_arguments)]
#![allow(deprecated)]

pub use cairo;
pub use ffi;
pub use gdk_pixbuf;
pub use gio;
pub use glib;
pub use pango;

// GDK 4 has no runtime to initialize
macro_rules! assert_initialized_main_thread {
    () => {};
}

// No-op
macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::match_same_arms)]
#[allow(clippy::let_and_return)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::wrong_self_convention)]
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::clone_on_copy)]
#[allow(clippy::cast_ptr_alignment)]
#[allow(unused_imports)]
mod auto;

mod alias;
pub mod prelude;
pub mod subclass;

mod cairo_interaction;
mod clipboard;
mod content_deserializer;
mod display;
mod draw_context;
mod drop;
mod event;
mod functions;
mod gl_texture;
mod keymap_key;
pub mod keys;
mod popup_layout;
mod rectangle;
mod rgba;
mod surface;
mod time_coord;
mod toplevel;

pub use self::auto::functions::*;
pub use auto::*;

pub use alias::*;
pub use functions::*;

pub use event::*;
pub use keymap_key::KeymapKey;
pub use popup_layout::PopupLayoutExtManual;
pub use rectangle::Rectangle;
pub use rgba::{RgbaParseError, RGBA};
pub use surface::SurfaceExtManual;
pub use time_coord::TimeCoord;

// This is the priority that the idle handler processing surface updates is given in the glib::MainLoop.
#[doc(alias = "GDK_PRIORITY_REDRAW")]
pub const PRIORITY_REDRAW: u32 = ffi::GDK_PRIORITY_REDRAW as u32;

// A mask covering all entries in ModifierType.
#[doc(alias = "GDK_MODIFIER_MASK")]
pub const MODIFIER_MASK: ModifierType = ModifierType::all();

// Defines all possible DND actions. This can be used in gdk::Drop::status messages when any drop can be accepted or a more specific drop method is not yet known.
#[doc(alias = "GDK_ACTION_ALL")]
pub const ACTION_ALL: u32 = ffi::GDK_ACTION_ALL as u32;

// Represents the current time, and can be used anywhere a time is expected.
#[doc(alias = "GDK_CURRENT_TIME")]
pub const CURRENT_TIME: u32 = ffi::GDK_CURRENT_TIME as u32;

/// The primary button. This is typically the left mouse button, or the right button in a left-handed setup.
#[doc(alias = "GDK_BUTTON_PRIMARY")]
pub const BUTTON_PRIMARY: u32 = ffi::GDK_BUTTON_PRIMARY as u32;

/// The middle button.
#[doc(alias = "GDK_BUTTON_MIDDLE")]
pub const BUTTON_MIDDLE: u32 = ffi::GDK_BUTTON_MIDDLE as u32;

/// The secondary button. This is typically the right mouse button, or the left button in a left-handed setup.
#[doc(alias = "GDK_BUTTON_SECONDARY")]
pub const BUTTON_SECONDARY: u32 = ffi::GDK_BUTTON_SECONDARY as u32;

// Used as the return value for stopping the propagation of an event handler.
#[doc(alias = "GDK_EVENT_STOP")]
pub const EVENT_STOP: u32 = ffi::GDK_EVENT_STOP as u32;

// Used as the return value for continuing the propagation of an event handler.
#[doc(alias = "GDK_EVENT_PROPAGATE")]
pub const EVENT_PROPAGATE: u32 = ffi::GDK_EVENT_PROPAGATE as u32;
