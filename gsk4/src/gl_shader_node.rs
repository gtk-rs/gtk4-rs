// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{GLShaderNode, RenderNode, RenderNodeType};
use glib::translate::*;

define_render_node!(
    GLShaderNode,
    ffi::GskGLShaderNode,
    RenderNodeType::GlShaderNode
);

impl GLShaderNode {
    #[doc(alias = "gsk_gl_shader_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self, idx: u32) -> Option<RenderNode> {
        assert!(idx < self.n_children());
        unsafe {
            from_glib_none(ffi::gsk_gl_shader_node_get_child(
                self.to_glib_none().0,
                idx,
            ))
        }
    }
}
