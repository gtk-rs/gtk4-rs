// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use graphene::{Point, Rect, Size};
use std::mem;

glib::wrapper! {
    #[doc(alias = "GskRoundedRect")]
    pub struct RoundedRect(BoxedInline<ffi::GskRoundedRect>);
}

impl RoundedRect {
    #[doc(alias = "gsk_rounded_rect_init")]
    pub fn new(
        bounds: Rect,
        top_left: Size,
        top_right: Size,
        bottom_right: Size,
        bottom_left: Size,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut rounded_rect = mem::MaybeUninit::uninit();
            ffi::gsk_rounded_rect_init(
                rounded_rect.as_mut_ptr(),
                bounds.to_glib_none().0,
                top_left.to_glib_none().0,
                top_right.to_glib_none().0,
                bottom_right.to_glib_none().0,
                bottom_left.to_glib_none().0,
            );
            Self::unsafe_from(rounded_rect.assume_init())
        }
    }

    #[doc(alias = "gsk_rounded_rect_init_from_rect")]
    #[doc(alias = "init_from_rect")]
    pub fn from_rect(bounds: Rect, radius: f32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut rounded_rect = mem::MaybeUninit::uninit();
            ffi::gsk_rounded_rect_init_from_rect(
                rounded_rect.as_mut_ptr(),
                bounds.to_glib_none().0,
                radius,
            );
            Self::unsafe_from(rounded_rect.assume_init())
        }
    }

    #[doc(alias = "gsk_rounded_rect_normalize")]
    pub fn normalize(&mut self) {
        unsafe {
            ffi::gsk_rounded_rect_normalize(&mut self.inner);
        }
    }

    #[doc(alias = "gsk_rounded_rect_offset")]
    pub fn offset(&mut self, dx: f32, dy: f32) {
        unsafe {
            ffi::gsk_rounded_rect_offset(&mut self.inner, dx, dy);
        }
    }

    #[doc(alias = "gsk_rounded_rect_shrink")]
    pub fn shrink(&mut self, top: f32, right: f32, bottom: f32, left: f32) {
        unsafe {
            ffi::gsk_rounded_rect_shrink(&mut self.inner, top, right, bottom, left);
        }
    }

    #[doc(alias = "gsk_rounded_rect_is_rectilinear")]
    pub fn is_rectilinear(&self) -> bool {
        unsafe { from_glib(ffi::gsk_rounded_rect_is_rectilinear(&self.inner)) }
    }

    #[doc(alias = "gsk_rounded_rect_contains_point")]
    pub fn contains_point(&self, point: Point) -> bool {
        unsafe {
            from_glib(ffi::gsk_rounded_rect_contains_point(
                &self.inner,
                point.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_rounded_rect_contains_rect")]
    pub fn contains_rect(&self, rect: Rect) -> bool {
        unsafe {
            from_glib(ffi::gsk_rounded_rect_contains_rect(
                &self.inner,
                rect.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_rounded_rect_intersects_rect")]
    pub fn intersects_rect(&self, rect: Rect) -> bool {
        unsafe {
            from_glib(ffi::gsk_rounded_rect_intersects_rect(
                &self.inner,
                rect.to_glib_none().0,
            ))
        }
    }

    #[inline]
    pub fn bounds(&self) -> &graphene::Rect {
        unsafe {
            &*(&self.inner.bounds as *const graphene::ffi::graphene_rect_t as *const graphene::Rect)
        }
    }

    #[inline]
    pub fn corner(&self) -> &[graphene::Size; 4] {
        unsafe {
            &*(&self.inner.corner as *const [graphene::ffi::graphene_size_t; 4]
                as *const [graphene::Size; 4])
        }
    }
}

impl std::fmt::Debug for RoundedRect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RoundedRect")
            .field("is_rectilinear", &self.is_rectilinear())
            .field("bounds", &self.bounds())
            .field("corner", &self.corner())
            .finish()
    }
}
