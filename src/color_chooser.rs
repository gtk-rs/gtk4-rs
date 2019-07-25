// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk::RGBA;
use gdk_sys;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use libc::c_int;
use ColorChooser;
use Orientation;

pub trait ColorChooserExtManual: 'static {
    fn add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: &[RGBA]);
}

impl<O: IsA<ColorChooser>> ColorChooserExtManual for O {
    fn add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: &[RGBA]) {
        unsafe {
            gtk_sys::gtk_color_chooser_add_palette(
                self.as_ref().to_glib_none().0,
                orientation.to_glib(),
                colors_per_line,
                colors.len() as c_int,
                colors.as_ptr() as *mut gdk_sys::GdkRGBA,
            )
        }
    }
}
