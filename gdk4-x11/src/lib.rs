// Copyright 2013-2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

#![allow(deprecated)]

#[macro_use]
extern crate glib;
extern crate gdk;
extern crate gdk_x11_sys;
extern crate gio;
extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;
extern crate x11;

pub(crate) use x11::xlib;

#[macro_use]
mod rt;

#[allow(unused_imports)]
#[allow(clippy::let_and_return)]
mod auto;

pub use auto::*;
