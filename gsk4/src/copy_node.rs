// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CopyNode, RenderNodeType};

define_render_node!(CopyNode, crate::ffi::GskCopyNode, RenderNodeType::CopyNode);

impl std::fmt::Debug for CopyNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CopyNode")
            .field("child", &self.child())
            .finish()
    }
}
