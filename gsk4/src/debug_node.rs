// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{DebugNode, RenderNodeType};

define_render_node!(DebugNode, ffi::GskDebugNode, RenderNodeType::DebugNode);

impl std::fmt::Debug for DebugNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DebugNode")
            .field("child", &self.child())
            .field("message", &self.message())
            .finish()
    }
}
