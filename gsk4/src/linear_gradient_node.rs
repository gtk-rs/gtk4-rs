// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{LinearGradientNode, RenderNodeType};

define_render_node!(
    LinearGradientNode,
    ffi::GskLinearGradientNode,
    RenderNodeType::LinearGradientNode
);

impl std::fmt::Debug for LinearGradientNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LinearGradientNode")
            .field("color_stops", &self.color_stops())
            .field("end", &self.end())
            .field("n_color_stops", &self.n_color_stops())
            .field("start", &self.start())
            .finish()
    }
}
