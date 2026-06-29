// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Side, SnapDirection, ffi};
use glib::translate::*;

#[derive(Debug, Clone, Copy)]
pub struct RectSnap(libc::c_uint);

impl FromGlib<libc::c_uint> for RectSnap {
    unsafe fn from_glib(value: libc::c_uint) -> Self {
        Self(value)
    }
}

impl IntoGlib for RectSnap {
    type GlibType = libc::c_uint;

    fn into_glib(self) -> libc::c_uint {
        self.0
    }
}

impl RectSnap {
    #[cfg(feature = "v4_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_24")))]
    #[doc(alias = "gsk_rect_snap_get_direction")]
    #[doc(alias = "get_direction")]
    pub fn direction(&self, side: Side) -> SnapDirection {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gsk_rect_snap_get_direction(
                self.into_glib(),
                side.into_glib(),
            ))
        }
    }

    #[cfg(feature = "v4_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_24")))]
    #[doc(alias = "gsk_rect_snap_new")]
    pub fn new(
        top: SnapDirection,
        right: SnapDirection,
        bottom: SnapDirection,
        left: SnapDirection,
    ) -> RectSnap {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gsk_rect_snap_new(
                top.into_glib(),
                right.into_glib(),
                bottom.into_glib(),
                left.into_glib(),
            ))
        }
    }
}
