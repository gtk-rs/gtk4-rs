// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::RGBA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GskColorStop")]
    pub struct ColorStop(BoxedInline<ffi::GskColorStop>);
}

impl ColorStop {
    pub fn new(offset: f32, color: RGBA) -> Self {
        assert_initialized_main_thread!();
        Self(ffi::GskColorStop {
            offset,
            color: unsafe { *color.to_glib_none().0 },
        })
    }

    pub fn offset(&self) -> f32 {
        self.0.offset
    }

    pub fn color(&self) -> RGBA {
        unsafe { from_glib_none(&self.0.color as *const _) }
    }
}

impl fmt::Debug for ColorStop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ColorStop")
            .field("offset", &self.offset())
            .field("color", &self.color())
            .finish()
    }
}

#[doc(hidden)]
impl FromGlibContainerAsVec<ffi::GskColorStop, *const ffi::GskColorStop> for ColorStop {
    unsafe fn from_glib_none_num_as_vec(ptr: *const ffi::GskColorStop, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(ColorStop(ptr::read(ptr.add(i))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(_: *const ffi::GskColorStop, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }

    unsafe fn from_glib_full_num_as_vec(_: *const ffi::GskColorStop, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }
}
