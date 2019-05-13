// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
#![cfg_attr(feature = "cargo-clippy", allow(transmute_int_to_char))]
#![cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ptr))]
#![cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]

extern crate glib_sys as glib_ffi;
extern crate gdk4_sys as gdk_ffi;
extern crate graphene_sys as graphene_ffi;
extern crate gsk4_sys as ffi;
#[macro_use]
extern crate glib;
extern crate gobject_sys as gobject_ffi;
extern crate cairo;
extern crate cairo_sys as cairo_ffi;
extern crate pango;
extern crate libc;
#[macro_use]
extern crate bitflags;

// GSK 4 has no runtime to initialize
macro_rules! assert_initialized_main_thread {
    () => ()
}

// No-op
macro_rules! skip_assert_initialized {
    () => ()
}

#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
mod auto;

pub mod prelude;

pub use auto::*;
pub use self::auto::functions::*;

pub use prelude::*;

pub use glib::Error;
