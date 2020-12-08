use crate::{RenderNode, Shadow, ShadowNode};
use glib::translate::*;
use glib::IsA;

impl ShadowNode {
    #[doc(alias = "gsk_shadow_node_new")]
    pub fn new<P: IsA<RenderNode>>(child: &P, shadows: &[Shadow]) -> ShadowNode {
        skip_assert_initialized!();
        let n_shadows = shadows.len() as usize;
        unsafe {
            from_glib_full(ffi::gsk_shadow_node_new(
                child.as_ref().to_glib_none().0,
                shadows.to_glib_none().0,
                n_shadows,
            ))
        }
    }

    #[doc(alias = "gsk_shadow_node_get_shadow")]
    pub fn get_shadow(&self, i: usize) -> Option<Shadow> {
        assert!(i < self.get_n_shadows());
        unsafe { from_glib_none(ffi::gsk_shadow_node_get_shadow(self.to_glib_none().0, i)) }
    }
}
