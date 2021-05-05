// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use graphene::{Point, Rect, Size};
use std::mem;

#[derive(Clone, Debug)]
pub struct RoundedRect(ffi::GskRoundedRect);

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
            Self(rounded_rect.assume_init())
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
            Self(rounded_rect.assume_init())
        }
    }

    #[doc(alias = "gsk_rounded_rect_init")]
    pub fn init(
        &mut self,
        bounds: Rect,
        top_left: Size,
        top_right: Size,
        bottom_right: Size,
        bottom_left: Size,
    ) {
        unsafe {
            ffi::gsk_rounded_rect_init(
                &mut self.0,
                bounds.to_glib_none().0,
                top_left.to_glib_none().0,
                top_right.to_glib_none().0,
                bottom_right.to_glib_none().0,
                bottom_left.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gsk_rounded_rect_init_from_rect")]
    pub fn init_from_rect(&mut self, bounds: Rect, radius: f32) {
        unsafe {
            ffi::gsk_rounded_rect_init_from_rect(&mut self.0, bounds.to_glib_none().0, radius);
        }
    }

    #[doc(alias = "gsk_rounded_rect_normalize")]
    pub fn normalize(&mut self) {
        unsafe {
            ffi::gsk_rounded_rect_normalize(&mut self.0);
        }
    }

    #[doc(alias = "gsk_rounded_rect_offset")]
    pub fn offset(&mut self, dx: f32, dy: f32) {
        unsafe {
            ffi::gsk_rounded_rect_offset(&mut self.0, dx, dy);
        }
    }

    #[doc(alias = "gsk_rounded_rect_shrink")]
    pub fn shrink(&mut self, top: f32, right: f32, bottom: f32, left: f32) {
        unsafe {
            ffi::gsk_rounded_rect_shrink(&mut self.0, top, right, bottom, left);
        }
    }

    #[doc(alias = "gsk_rounded_rect_is_rectilinear")]
    pub fn is_rectilinear(&self) -> bool {
        unsafe { from_glib(ffi::gsk_rounded_rect_is_rectilinear(&self.0)) }
    }

    #[doc(alias = "gsk_rounded_rect_contains_point")]
    pub fn contains_point(&self, point: Point) -> bool {
        unsafe {
            from_glib(ffi::gsk_rounded_rect_contains_point(
                &self.0,
                point.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_rounded_rect_contains_rect")]
    pub fn contains_rect(&self, rect: Rect) -> bool {
        unsafe {
            from_glib(ffi::gsk_rounded_rect_contains_rect(
                &self.0,
                rect.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_rounded_rect_intersects_rect")]
    pub fn intersects_rect(&self, rect: Rect) -> bool {
        unsafe {
            from_glib(ffi::gsk_rounded_rect_intersects_rect(
                &self.0,
                rect.to_glib_none().0,
            ))
        }
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GskRoundedRect> for RoundedRect {
    unsafe fn from_glib_none(ptr: *const ffi::GskRoundedRect) -> Self {
        Self(*ptr)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GskRoundedRect> for RoundedRect {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<*const ffi::GskRoundedRect, Self> {
        Stash(&self.0, self)
    }
}
