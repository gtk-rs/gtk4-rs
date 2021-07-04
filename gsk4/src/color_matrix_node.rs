// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNode, RenderNodeType};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GskColorMatrixNode")]
    pub struct ColorMatrixNode(Shared<ffi::GskColorMatrixNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

define_render_node!(
    ColorMatrixNode,
    ffi::GskColorMatrixNode,
    ffi::gsk_color_matrix_node_get_type,
    RenderNodeType::ColorMatrixNode
);

impl ColorMatrixNode {
    #[doc(alias = "gsk_color_matrix_node_new")]
    pub fn new<P: AsRef<RenderNode>>(
        child: &P,
        color_matrix: &graphene::Matrix,
        color_offset: &graphene::Vec4,
    ) -> Self {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_color_matrix_node_new(
                child.as_ref().to_glib_none().0,
                color_matrix.to_glib_none().0,
                color_offset.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_color_matrix_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_color_matrix_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_color_matrix_node_get_color_matrix")]
    #[doc(alias = "get_color_matrix")]
    pub fn color_matrix(&self) -> Option<graphene::Matrix> {
        unsafe {
            from_glib_none(ffi::gsk_color_matrix_node_get_color_matrix(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_color_matrix_node_get_color_offset")]
    #[doc(alias = "get_color_offset")]
    pub fn color_offset(&self) -> Option<graphene::Vec4> {
        unsafe {
            from_glib_none(ffi::gsk_color_matrix_node_get_color_offset(
                self.to_glib_none().0,
            ))
        }
    }
}
