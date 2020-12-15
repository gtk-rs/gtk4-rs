// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::fmt;
use std::ops;

glib::glib_wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Border(Boxed<ffi::GtkBorder>);

    match fn {
        copy => |ptr| ffi::gtk_border_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_border_free(ptr),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || ffi::gtk_border_get_type(),
    }
}

impl ops::Deref for Border {
    type Target = ffi::GtkBorder;

    fn deref(&self) -> &Self::Target {
        &(*self.0)
    }
}

impl ops::DerefMut for Border {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut (*self.0)
    }
}

impl Border {
    pub fn new() -> Border {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_border_new()) }
    }
}

impl Default for Border {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Border {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.debug_struct("Border")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("top", &self.top)
            .field("bottom", &self.bottom)
            .finish()
    }
}
