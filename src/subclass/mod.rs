// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

//pub mod application;
pub mod application_window;
pub mod box_;
pub mod dialog;
pub mod header_bar;
pub mod widget;
pub mod window;

pub mod prelude {
    //pub use super::application::GtkApplicationImpl;
    pub use super::application_window::ApplicationWindowImpl;
    pub use super::box_::BoxImpl;
    pub use super::dialog::DialogImpl;
    pub use super::header_bar::HeaderBarImpl;
    pub use super::widget::WidgetImpl;
    pub use super::window::WindowImpl;
    pub use gio::subclass::prelude::*;
    pub use glib::subclass::prelude::*;
}
