// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RadialGradientNode, RenderNodeType};

define_render_node!(
    RadialGradientNode,
    ffi::GskRadialGradientNode,
    RenderNodeType::RadialGradientNode
);

impl std::fmt::Debug for RadialGradientNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RadialGradientNode")
            .field("center", &self.center())
            .field("color_stops", &self.color_stops())
            .field("end", &self.end())
            .field("hradius", &self.hradius())
            .field("n_color_stops", &self.n_color_stops())
            .field("start", &self.start())
            .field("vradius", &self.vradius())
            .finish()
    }
}
