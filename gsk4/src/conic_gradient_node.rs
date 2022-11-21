// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ConicGradientNode, RenderNodeType};

define_render_node!(
    ConicGradientNode,
    ffi::GskConicGradientNode,
    RenderNodeType::ConicGradientNode
);

impl std::fmt::Debug for ConicGradientNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[cfg(feature = "v4_2")]
        {
            f.debug_struct("ConicGradientNode")
                .field("angle", &self.angle())
                .field("center", &self.center())
                .field("color_stops", &self.color_stops())
                .field("n_color_stops", &self.n_color_stops())
                .field("rotation", &self.rotation())
                .finish()
        }
        #[cfg(not(feature = "v4_2"))]
        {
            f.debug_struct("ConicGradientNode")
                .field("center", &self.center())
                .field("color_stops", &self.color_stops())
                .field("n_color_stops", &self.n_color_stops())
                .field("rotation", &self.rotation())
                .finish()
        }
    }
}
