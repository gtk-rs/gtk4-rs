// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CairoNode, RenderNodeType};

define_render_node!(CairoNode, ffi::GskCairoNode, RenderNodeType::CairoNode);

impl std::fmt::Debug for CairoNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CairoNode")
            .field("draw_context", &self.draw_context())
            .field("surface", &self.surface())
            .finish()
    }
}
