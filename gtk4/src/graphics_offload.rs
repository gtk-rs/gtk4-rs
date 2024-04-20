// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{prelude::*, GraphicsOffload, Widget};

impl GraphicsOffload {
    #[doc(alias = "gtk_graphics_offload_new")]
    pub fn new(child: Option<&impl IsA<Widget>>) -> GraphicsOffload {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_graphics_offload_new(
                child.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
impl Default for GraphicsOffload {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}
