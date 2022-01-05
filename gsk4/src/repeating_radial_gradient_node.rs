// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, RepeatingRadialGradientNode};

define_render_node!(
    RepeatingRadialGradientNode,
    ffi::GskRepeatingRadialGradientNode,
    RenderNodeType::RepeatingRadialGradientNode
);

impl std::fmt::Debug for RepeatingRadialGradientNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RepeatingRadialGradientNode").finish()
    }
}
