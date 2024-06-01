// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{FillNode, RenderNodeType};

define_render_node!(FillNode, crate::ffi::GskFillNode, RenderNodeType::FillNode);

impl std::fmt::Debug for FillNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FillNode")
            .field("child", &self.child())
            .field("fill_rule", &self.fill_rule())
            .field("path", &self.path())
            .finish()
    }
}
