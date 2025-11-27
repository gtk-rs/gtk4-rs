// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{PasteNode, RenderNodeType};

define_render_node!(
    PasteNode,
    crate::ffi::GskPasteNode,
    RenderNodeType::PasteNode
);

impl std::fmt::Debug for PasteNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PasteNode")
            .field("depth", &self.depth())
            .finish()
    }
}
