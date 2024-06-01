// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`SectionModel`](crate::SectionModel)
//! interface.

use glib::translate::*;

use crate::{ffi, prelude::*, subclass::prelude::*, SectionModel};

pub trait SectionModelImpl: ListModelImpl {
    #[doc(alias = "get_section")]
    fn section(&self, position: u32) -> (u32, u32) {
        self.parent_section(position)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::SectionModelImplExt> Sealed for T {}
}

pub trait SectionModelImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_section(&self, position: u32) -> (u32, u32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<SectionModel>()
                as *const ffi::GtkSectionModelInterface;

            let func = (*parent_iface)
                .get_section
                .expect("no parent \"get_section\" implementation");

            let mut start = std::mem::MaybeUninit::uninit();
            let mut end = std::mem::MaybeUninit::uninit();
            func(
                self.obj()
                    .unsafe_cast_ref::<SectionModel>()
                    .to_glib_none()
                    .0,
                position,
                start.as_mut_ptr(),
                end.as_mut_ptr(),
            );
            (start.assume_init(), end.assume_init())
        }
    }
}

impl<T: SectionModelImpl> SectionModelImplExt for T {}

unsafe impl<T: SectionModelImpl> IsImplementable<T> for SectionModel {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        assert_initialized_main_thread!();

        iface.get_section = Some(model_get_section::<T>);
    }
}

unsafe extern "C" fn model_get_section<T: SectionModelImpl>(
    model: *mut ffi::GtkSectionModel,
    position: u32,
    startptr: *mut libc::c_uint,
    endptr: *mut libc::c_uint,
) {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.imp();

    let (start, end) = imp.section(position);
    *startptr = start;
    *endptr = end;
}
