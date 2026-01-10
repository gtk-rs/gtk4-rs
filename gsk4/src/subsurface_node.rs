// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, SubsurfaceNode};

define_render_node!(
    SubsurfaceNode,
    crate::ffi::GskSubsurfaceNode,
    RenderNodeType::SubsurfaceNode
);

impl std::fmt::Debug for SubsurfaceNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubsurfaceNode").finish()
    }
}
