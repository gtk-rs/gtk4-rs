// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNode, RenderNodeType, Transform};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GskTransformNode")]
    pub struct TransformNode(Shared<ffi::GskTransformNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

define_render_node!(
    TransformNode,
    ffi::GskTransformNode,
    ffi::gsk_transform_node_get_type,
    RenderNodeType::TransformNode
);

impl TransformNode {
    #[doc(alias = "gsk_transform_node_new")]
    pub fn new<P: AsRef<RenderNode>>(child: &P, transform: &Transform) -> Self {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_transform_node_new(
                child.as_ref().to_glib_none().0,
                transform.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_transform_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_transform_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_transform_node_get_transform")]
    #[doc(alias = "get_transform")]
    pub fn transform(&self) -> Option<Transform> {
        unsafe { from_glib_none(ffi::gsk_transform_node_get_transform(self.to_glib_none().0)) }
    }
}
