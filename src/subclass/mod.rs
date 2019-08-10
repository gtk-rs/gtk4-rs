// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

pub mod application;
pub mod container;
pub mod widget;

pub mod prelude {
    pub use super::application::GtkApplicationImpl;
    pub use super::widget::WidgetImpl;
    pub use gio::subclass::prelude::*;
    pub use glib::subclass::prelude::*;
}
