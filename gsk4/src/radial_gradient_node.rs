// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RadialGradientNode, RenderNodeType};

define_render_node!(
    RadialGradientNode,
    crate::ffi::GskRadialGradientNode,
    RenderNodeType::RadialGradientNode
);

impl RadialGradientNode {
    #[cfg(feature = "v4_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_24")))]
    #[doc(alias = "gsk_radial_gradient_node_get_snap")]
    #[doc(alias = "get_snap")]
    pub fn snap(&self) -> crate::RectSnap {
        unsafe {
            glib::translate::from_glib(crate::ffi::gsk_radial_gradient_node_get_snap(
                glib::translate::ToGlibPtr::to_glib_none(self).0,
            ))
        }
    }
}

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
