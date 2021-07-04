// Take a look at the license at the top of the repository in the LICENSE file.

use glib::ffi::gpointer;
use glib::translate::*;

use std::{mem, ptr};

#[repr(transparent)]
#[doc(alias = "GtkTreeIter")]
pub struct TreeIter(ffi::GtkTreeIter);

impl TreeIter {
    /// Create a TreeIter storing an integer as user_data
    pub fn new(stamp: i32, user_data: i32) -> Self {
        assert_initialized_main_thread!();

        // We only really need the first user_data field for our data.
        let user_data = user_data as gpointer;
        let user_data2 = ptr::null::<gpointer>() as gpointer;
        let user_data3 = ptr::null::<gpointer>() as gpointer;

        Self {
            0: ffi::GtkTreeIter {
                stamp,
                user_data,
                user_data2,
                user_data3,
            },
        }
    }

    /// The stamp should be a unique identifier of the model.
    /// If you implement a model you need to check if a TreeIter belongs to you by checking the
    /// stamp value.
    pub fn stamp(&self) -> i32 {
        self.0.stamp
    }

    /// Retrieve the stored user data
    pub fn user_data(&self) -> i32 {
        self.0.user_data as i32
    }
}

#[doc(hidden)]
impl Uninitialized for TreeIter {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GtkTreeIter> for TreeIter {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GtkTreeIter, Self> {
        let ptr: *const TreeIter = &*self;
        Stash(ptr as *const ffi::GtkTreeIter, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GtkTreeIter> for TreeIter {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GtkTreeIter, Self> {
        let ptr: *mut TreeIter = &mut *self;
        StashMut(ptr as *mut ffi::GtkTreeIter, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::GtkTreeIter> for TreeIter {
    unsafe fn from_glib_borrow(ptr: *mut ffi::GtkTreeIter) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(Self {
            0: ffi::GtkTreeIter {
                stamp: (*ptr).stamp,
                user_data: (*ptr).user_data,
                user_data2: (*ptr).user_data2,
                user_data3: (*ptr).user_data3,
            },
        })
    }
}
