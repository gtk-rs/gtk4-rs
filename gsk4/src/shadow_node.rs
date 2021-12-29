// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, Shadow, ShadowNode};
use glib::translate::*;
define_render_node!(ShadowNode, ffi::GskShadowNode, RenderNodeType::ShadowNode);

impl ShadowNode {
    #[doc(alias = "gsk_shadow_node_get_shadow")]
    #[doc(alias = "get_shadow")]
    pub fn shadow(&self, i: usize) -> Option<Shadow> {
        assert!(i < self.n_shadows());
        unsafe { from_glib_none(ffi::gsk_shadow_node_get_shadow(self.to_glib_none().0, i)) }
    }
}
