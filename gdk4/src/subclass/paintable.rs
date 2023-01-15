// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Paintable`](crate::Paintable) interface.

use crate::{prelude::*, subclass::prelude::*, Paintable, PaintableFlags, Snapshot};
use glib::translate::*;

pub trait PaintableImpl: ObjectImpl {
    #[doc(alias = "get_current_image")]
    fn current_image(&self) -> Paintable {
        self.parent_current_image()
    }

    #[doc(alias = "get_flags")]
    fn flags(&self) -> PaintableFlags {
        self.parent_flags()
    }

    #[doc(alias = "get_intrinsic_width")]
    fn intrinsic_width(&self) -> i32 {
        self.parent_intrinsic_width()
    }

    #[doc(alias = "get_intrinsic_height")]
    fn intrinsic_height(&self) -> i32 {
        self.parent_intrinsic_height()
    }

    #[doc(alias = "get_intrinsic_aspect_ratio")]
    fn intrinsic_aspect_ratio(&self) -> f64 {
        self.parent_intrinsic_aspect_ratio()
    }

    fn snapshot(&self, snapshot: &Snapshot, width: f64, height: f64);
}

pub trait PaintableImplExt: ObjectSubclass {
    fn parent_current_image(&self) -> Paintable;
    fn parent_flags(&self) -> PaintableFlags;
    fn parent_intrinsic_width(&self) -> i32;
    fn parent_intrinsic_height(&self) -> i32;
    fn parent_intrinsic_aspect_ratio(&self) -> f64;
    fn parent_snapshot(&self, snapshot: &Snapshot, width: f64, height: f64);
}

impl<T: PaintableImpl> PaintableImplExt for T {
    fn parent_current_image(&self) -> Paintable {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Paintable>()
                as *const ffi::GdkPaintableInterface;
            let func = (*parent_iface)
                .get_current_image
                .expect("no parent \"get_current_image\" implementation");

            let ret = func(self.obj().unsafe_cast_ref::<Paintable>().to_glib_none().0);

            from_glib_full(ret)
        }
    }

    fn parent_flags(&self) -> PaintableFlags {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Paintable>()
                as *const ffi::GdkPaintableInterface;
            let func = (*parent_iface)
                .get_flags
                .expect("no parent \"get_flags\" implementation");

            from_glib(func(
                self.obj().unsafe_cast_ref::<Paintable>().to_glib_none().0,
            ))
        }
    }

    fn parent_intrinsic_width(&self) -> i32 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Paintable>()
                as *const ffi::GdkPaintableInterface;
            let func = (*parent_iface)
                .get_intrinsic_width
                .expect("no parent \"get_intrinsic_width\" implementation");

            func(self.obj().unsafe_cast_ref::<Paintable>().to_glib_none().0)
        }
    }

    fn parent_intrinsic_height(&self) -> i32 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Paintable>()
                as *const ffi::GdkPaintableInterface;
            let func = (*parent_iface)
                .get_intrinsic_height
                .expect("no parent \"get_intrinsic_height\" implementation");

            func(self.obj().unsafe_cast_ref::<Paintable>().to_glib_none().0)
        }
    }

    fn parent_intrinsic_aspect_ratio(&self) -> f64 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Paintable>()
                as *const ffi::GdkPaintableInterface;
            let func = (*parent_iface)
                .get_intrinsic_aspect_ratio
                .expect("no parent \"get_intrinsic_aspect_ratio\" implementation");

            func(self.obj().unsafe_cast_ref::<Paintable>().to_glib_none().0)
        }
    }

    fn parent_snapshot(&self, snapshot: &Snapshot, width: f64, height: f64) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Paintable>()
                as *const ffi::GdkPaintableInterface;
            let func = (*parent_iface)
                .snapshot
                .expect("no parent \"snapshot\" implementation");

            func(
                self.obj().unsafe_cast_ref::<Paintable>().to_glib_none().0,
                snapshot.to_glib_none().0,
                width,
                height,
            )
        }
    }
}

unsafe impl<T: PaintableImpl> IsImplementable<T> for Paintable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.get_current_image = Some(paintable_get_current_image::<T>);
        iface.get_flags = Some(paintable_get_flags::<T>);
        iface.get_intrinsic_width = Some(paintable_get_intrinsic_width::<T>);
        iface.get_intrinsic_height = Some(paintable_get_intrinsic_height::<T>);
        iface.get_intrinsic_aspect_ratio = Some(paintable_get_intrinsic_aspect_ratio::<T>);
        iface.snapshot = Some(paintable_snapshot::<T>);
    }
}

unsafe extern "C" fn paintable_get_current_image<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> *mut ffi::GdkPaintable {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.imp();

    imp.current_image().into_glib_ptr()
}

unsafe extern "C" fn paintable_get_flags<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> ffi::GdkPaintableFlags {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.imp();

    imp.flags().into_glib()
}

unsafe extern "C" fn paintable_get_intrinsic_width<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> i32 {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.imp();

    imp.intrinsic_width()
}

unsafe extern "C" fn paintable_get_intrinsic_height<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> i32 {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.imp();

    imp.intrinsic_height()
}

unsafe extern "C" fn paintable_get_intrinsic_aspect_ratio<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> f64 {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.imp();

    imp.intrinsic_aspect_ratio()
}

unsafe extern "C" fn paintable_snapshot<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
    snapshotptr: *mut ffi::GdkSnapshot,
    width: f64,
    height: f64,
) {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.imp();

    imp.snapshot(&Snapshot::from_glib_borrow(snapshotptr), width, height)
}
