// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CrossFadeNode, RenderNodeType};

define_render_node!(
    CrossFadeNode,
    ffi::GskCrossFadeNode,
    RenderNodeType::CrossFadeNode
);

impl std::fmt::Debug for CrossFadeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CrossFadeNode")
            .field("start_child", &self.start_child())
            .field("end_child", &self.end_child())
            .field("progress", &self.progress())
            .finish()
    }
}
