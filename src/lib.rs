// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
#![cfg_attr(feature = "cargo-clippy", allow(transmute_int_to_char))]
#![cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ptr))]
#![cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]

extern crate gdk4_sys as gdk_sys;
extern crate glib_sys;
extern crate graphene_sys;
extern crate gsk4_sys as gsk_sys;
#[macro_use]
extern crate glib;
extern crate cairo;
extern crate cairo_sys;
extern crate gobject_sys;
extern crate libc;
extern crate pango;
#[macro_use]
extern crate bitflags;

extern crate gdk4 as gdk;
extern crate graphene;

// GSK 4 has no runtime to initialize
macro_rules! assert_initialized_main_thread {
    () => {};
}

// No-op
macro_rules! skip_assert_initialized {
    () => {};
}

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
mod auto;

pub mod prelude;

pub use self::auto::functions::*;
pub use auto::*;

mod color_stop;
mod render_nodes;
mod rounded_rect;
mod shadow;

pub use color_stop::ColorStop;
pub use render_nodes::*;
pub use rounded_rect::RoundedRect;
pub use shadow::Shadow;
