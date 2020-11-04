use glib::translate::*;
use gtk_sys;
use std::mem;

#[repr(C)]
pub struct Requisition {
    pub width: i32,
    pub height: i32,
}

#[doc(hidden)]
impl Uninitialized for Requisition {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::uninitialized()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const gtk_sys::GtkRequisition> for Requisition {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const gtk_sys::GtkRequisition, Self> {
        let ptr: *const Requisition = &*self;
        Stash(ptr as *const gtk_sys::GtkRequisition, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut gtk_sys::GtkRequisition> for Requisition {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut gtk_sys::GtkRequisition, Self> {
        let ptr: *mut Requisition = &mut *self;
        StashMut(ptr as *mut gtk_sys::GtkRequisition, self)
    }
}
