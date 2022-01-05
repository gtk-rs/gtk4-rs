// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, RepeatingLinearGradientNode};

define_render_node!(
    RepeatingLinearGradientNode,
    ffi::GskRepeatingLinearGradientNode,
    RenderNodeType::RepeatingLinearGradientNode
);

impl std::fmt::Debug for RepeatingLinearGradientNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RepeatingLinearGradientNode").finish()
    }
}
