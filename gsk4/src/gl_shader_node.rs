// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RenderNode;
use crate::{GLShader, RenderNodeType};
use glib::translate::*;

define_render_node!(
    GLShaderNode,
    ffi::GskGLShaderNode,
    ffi::gsk_gl_shader_node_get_type,
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

    #[doc(alias = "gsk_gl_shader_node_get_args")]
    #[doc(alias = "get_args")]
    pub fn args(&self) -> Option<glib::Bytes> {
        unsafe { from_glib_none(ffi::gsk_gl_shader_node_get_args(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_gl_shader_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self, idx: u32) -> Option<RenderNode> {
        unsafe {
            from_glib_none(ffi::gsk_gl_shader_node_get_child(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    #[doc(alias = "gsk_gl_shader_node_get_n_children")]
    #[doc(alias = "get_n_children")]
    pub fn n_children(&self) -> u32 {
        unsafe { ffi::gsk_gl_shader_node_get_n_children(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_gl_shader_node_get_shader")]
    #[doc(alias = "get_shader")]
    pub fn shader(&self) -> Option<GLShader> {
        unsafe { from_glib_none(ffi::gsk_gl_shader_node_get_shader(self.to_glib_none().0)) }
    }
}
