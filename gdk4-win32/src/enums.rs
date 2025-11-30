// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "v4_20")]
use glib::translate::*;

#[cfg(feature = "v4_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_20")))]
impl glib::error::ErrorDomain for crate::D3D12Error {
    #[inline]
    fn domain() -> glib::Quark {
        unsafe { from_glib(crate::ffi::gdk_d3d12_error_quark()) }
    }

    #[inline]
    fn code(self) -> i32 {
        self.into_glib()
    }

    #[inline]
    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();

        unsafe { Some(from_glib(code)) }
    }
}
