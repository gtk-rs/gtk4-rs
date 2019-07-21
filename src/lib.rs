// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
#![cfg_attr(feature = "cargo-clippy", allow(transmute_int_to_char))]
#![cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ptr))]
#![cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]

extern crate glib_sys;
extern crate gio_sys;
extern crate gdk_pixbuf;
extern crate gdk4_sys as gdk_sys;
extern crate gio;
#[macro_use]
extern crate glib;
extern crate gobject_sys;
extern crate cairo;
extern crate cairo_sys;
extern crate pango;
extern crate libc;
#[macro_use]
extern crate bitflags;

// GDK 4 has no runtime to initialize
macro_rules! assert_initialized_main_thread {
    () => ()
}

// No-op
macro_rules! skip_assert_initialized {
    () => ()
}

#[allow(unused_imports)]
use glib::{
    Cast,
    Continue,
    Error,
    IsA,
    Object,
    StaticType,
    ToValue,
    Type,
    TypedValue,
    Value,
};

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
mod auto;

mod alias;
pub mod prelude;

mod clipboard;
mod content_deserializer;
mod draw_context;
mod drop;
mod events;
mod functions;
mod geometry;
mod keymap;
mod keymap_key;
mod rgba;
mod surface;
mod time_coord;

pub use auto::*;
pub use self::auto::functions::*;

pub use alias::*;
pub use functions::*;
pub use prelude::*;

pub use events::*;
pub use geometry::Geometry;
pub use keymap_key::KeymapKey;
pub use rgba::{RGBA, RgbaParseError};
pub use surface::SurfaceExtManual;
pub use time_coord::TimeCoord;
