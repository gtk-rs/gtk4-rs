
use glib::translate::*;
use std::fmt;
use crate::{RenderNode, Shadow, ShadowNode};

impl ShadowNode {
    pub fn new<P: IsA<RenderNode>>(child: &P, shadows: &[Shadow]) -> ShadowNode {
        skip_assert_initialized!();
        let n_shadows = shadows.len() as usize;
        unsafe {
            from_glib_full(ffi::gsk_shadow_node_new(
                child.as_ref().to_glib_none().0,
                shadows.to_glib_none().0,
                n_shadows,
            ))
        }
    }
}
