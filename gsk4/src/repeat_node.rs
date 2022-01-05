// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, RepeatNode};

define_render_node!(RepeatNode, ffi::GskRepeatNode, RenderNodeType::RepeatNode);

impl std::fmt::Debug for RepeatNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RepeatNode")
            .field("child_bounds", &self.child_bounds())
            .field("child", &self.child())
            .finish()
    }
}
