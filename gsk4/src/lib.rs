// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

#![allow(clippy::let_unit_value)]
#![allow(clippy::new_without_default)]
#![allow(clippy::type_complexity)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::too_many_arguments)]
#![allow(deprecated)]

extern crate gdk4_sys as gdk_sys;
extern crate glib_sys;
extern crate graphene_sys;
extern crate gsk4_sys as gsk_sys;
#[macro_use]
extern crate glib;
extern crate bitflags;
extern crate cairo;
extern crate cairo_sys;
extern crate gobject_sys;
extern crate libc;
extern crate pango;

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

#[allow(clippy::match_same_arms)]
#[allow(clippy::let_and_return)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::wrong_self_convention)]
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::clone_on_copy)]
#[allow(clippy::cast_ptr_alignment)]
#[allow(unused_imports)]
mod auto;

pub mod prelude;

pub use auto::*;

mod border_node;
mod color_stop;
mod rounded_rect;
mod shadow;

pub use border_node::BorderNodeManualExt;
pub use color_stop::ColorStop;
pub use rounded_rect::RoundedRect;
pub use shadow::Shadow;
