// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ConicGradientNode, RenderNodeType};

define_render_node!(
    ConicGradientNode,
    crate::ffi::GskConicGradientNode,
    RenderNodeType::ConicGradientNode
);

impl ConicGradientNode {
    #[cfg(feature = "v4_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_24")))]
    #[doc(alias = "gsk_conic_gradient_node_get_snap")]
    #[doc(alias = "get_snap")]
    pub fn snap(&self) -> crate::RectSnap {
        unsafe {
            glib::translate::from_glib(crate::ffi::gsk_conic_gradient_node_get_snap(
                glib::translate::ToGlibPtr::to_glib_none(self).0,
            ))
        }
    }
}

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
