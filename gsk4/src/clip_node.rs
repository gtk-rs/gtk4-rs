// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ClipNode, RenderNodeType};

define_render_node!(ClipNode, ffi::GskClipNode, RenderNodeType::ClipNode);

impl std::fmt::Debug for ClipNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ClipNode")
            .field("child", &self.child())
            .field("clip", &self.clip())
            .finish()
    }
}
