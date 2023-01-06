// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, ColorChooser, Orientation};
use gdk::RGBA;
use glib::translate::*;
use libc::c_int;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`ColorChooser`](crate::ColorChooser).
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait ColorChooserExtManual: 'static {
    #[doc(alias = "gtk_color_chooser_add_palette")]
    fn add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: &[RGBA]);
}

impl<O: IsA<ColorChooser>> ColorChooserExtManual for O {
    fn add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: &[RGBA]) {
        unsafe {
            ffi::gtk_color_chooser_add_palette(
                self.as_ref().to_glib_none().0,
                orientation.into_glib(),
                colors_per_line,
                colors.len() as c_int,
                if colors.is_empty() {
                    std::ptr::null_mut()
                } else {
                    colors.as_ptr() as *mut gdk::ffi::GdkRGBA
                },
            )
        }
    }
}
