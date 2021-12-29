// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RenderNode;
use crate::{GLShader, GLShaderNode, RenderNodeType};
use glib::translate::*;

define_render_node!(
    GLShaderNode,
    ffi::GskGLShaderNode,
    RenderNodeType::GlShaderNode
);

impl GLShaderNode {
    #[doc(alias = "gsk_gl_shader_node_new")]
    pub fn new(
        shader: &GLShader,
        bounds: &graphene::Rect,
        args: &glib::Bytes,
        children: &[RenderNode],
    ) -> Self {
        skip_assert_initialized!();
        let n_children = children.len() as u32;
        unsafe {
            from_glib_full(ffi::gsk_gl_shader_node_new(
                shader.to_glib_none().0,
                bounds.to_glib_none().0,
                args.to_glib_none().0,
                children.to_glib_none().0,
                n_children,
            ))
        }
    }
}
