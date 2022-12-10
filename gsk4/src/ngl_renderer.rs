// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Renderer};
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskNglRenderer")]
    pub struct NglRenderer(Object<ffi::GskNglRenderer>) @extends Renderer;

    match fn {
        type_ => || ffi::gsk_ngl_renderer_get_type(),
    }
}

impl NglRenderer {
    #[doc(alias = "gsk_ngl_renderer_new")]
    pub fn new() -> NglRenderer {
        assert_initialized_main_thread!();
        unsafe { Renderer::from_glib_full(ffi::gsk_ngl_renderer_new()).unsafe_cast() }
    }
}

impl Default for NglRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NglRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NglRenderer")
    }
}
