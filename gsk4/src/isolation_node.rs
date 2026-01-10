// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{IsolationNode, RenderNodeType};

define_render_node!(
    IsolationNode,
    crate::ffi::GskIsolationNode,
    RenderNodeType::IsolationNode
);

impl std::fmt::Debug for IsolationNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IsolationNode")
            .field("isolations", &self.isolations())
            .finish()
    }
}
