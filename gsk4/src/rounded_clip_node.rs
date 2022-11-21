// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, RoundedClipNode};

define_render_node!(
    RoundedClipNode,
    ffi::GskRoundedClipNode,
    RenderNodeType::RoundedClipNode
);

impl std::fmt::Debug for RoundedClipNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RoundedClipNode")
            .field("clip", &self.clip())
            .field("child", &self.child())
            .finish()
    }
}
