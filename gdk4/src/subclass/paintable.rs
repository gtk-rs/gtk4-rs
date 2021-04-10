// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Paintable, PaintableFlags, Snapshot};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait PaintableImpl: ObjectImpl {
    fn current_image(&self, paintable: &Self::Type) -> Paintable {
        self.parent_current_image(paintable)
    }

    fn flags(&self, paintable: &Self::Type) -> PaintableFlags {
        self.parent_flags(paintable)
    }

    fn intrinsic_width(&self, paintable: &Self::Type) -> i32 {
        self.parent_intrinsic_width(paintable)
    }

    fn intrinsic_height(&self, paintable: &Self::Type) -> i32 {
        self.parent_intrinsic_height(paintable)
    }

    fn intrinsic_aspect_ratio(&self, paintable: &Self::Type) -> f64 {
        self.parent_intrinsic_aspect_ratio(paintable)
    }

    fn snapshot(&self, paintable: &Self::Type, snapshot: &Snapshot, width: f64, height: f64);
}

pub trait PaintableImplExt: ObjectSubclass {
    fn parent_current_image(&self, paintable: &Self::Type) -> Paintable;
    fn parent_flags(&self, paintable: &Self::Type) -> PaintableFlags;
    fn parent_intrinsic_width(&self, paintable: &Self::Type) -> i32;
    fn parent_intrinsic_height(&self, paintable: &Self::Type) -> i32;
    fn parent_intrinsic_aspect_ratio(&self, paintable: &Self::Type) -> f64;
    fn parent_snapshot(&self, paintable: &Self::Type, snapshot: &Snapshot, width: f64, height: f64);
}

impl<T: PaintableImpl> PaintableImplExt for T {
    fn parent_current_image(&self, paintable: &Self::Type) -> Paintable {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Paintable>()
                as *const ffi::GdkPaintableInterface;
            let func = (*parent_iface)
                .get_current_image
                .expect("no parent \"get_current_image\" implementation");

            let ret = func(paintable.unsafe_cast_ref::<Paintable>().to_glib_none().0);

            from_glib_full(ret)
        }
    }

    fn parent_flags(&self, paintable: &Self::Type) -> PaintableFlags {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Paintable>()
                as *const ffi::GdkPaintableInterface;
            let func = (*parent_iface)
                .get_flags
                .expect("no parent \"get_flags\" implementation");

            from_glib(func(
                paintable.unsafe_cast_ref::<Paintable>().to_glib_none().0,
            ))
        }
    }

    fn parent_intrinsic_width(&self, paintable: &Self::Type) -> i32 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Paintable>()
                as *const ffi::GdkPaintableInterface;
            let func = (*parent_iface)
                .get_intrinsic_width
                .expect("no parent \"get_intrinsic_width\" implementation");

            func(paintable.unsafe_cast_ref::<Paintable>().to_glib_none().0)
        }
    }

    fn parent_intrinsic_height(&self, paintable: &Self::Type) -> i32 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Paintable>()
                as *const ffi::GdkPaintableInterface;
            let func = (*parent_iface)
                .get_intrinsic_height
                .expect("no parent \"get_intrinsic_height\" implementation");

            func(paintable.unsafe_cast_ref::<Paintable>().to_glib_none().0)
        }
    }

    fn parent_intrinsic_aspect_ratio(&self, paintable: &Self::Type) -> f64 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Paintable>()
                as *const ffi::GdkPaintableInterface;
            let func = (*parent_iface)
                .get_intrinsic_aspect_ratio
                .expect("no parent \"get_intrinsic_aspect_ratio\" implementation");

            func(paintable.unsafe_cast_ref::<Paintable>().to_glib_none().0)
        }
    }

    fn parent_snapshot(
        &self,
        paintable: &Self::Type,
        snapshot: &Snapshot,
        width: f64,
        height: f64,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<Paintable>()
                as *const ffi::GdkPaintableInterface;
            let func = (*parent_iface)
                .snapshot
                .expect("no parent \"snapshot\" implementation");

            func(
                paintable.unsafe_cast_ref::<Paintable>().to_glib_none().0,
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

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn paintable_get_current_image<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> *mut ffi::GdkPaintable {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.current_image(from_glib_borrow::<_, Paintable>(paintable).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn paintable_get_flags<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> ffi::GdkPaintableFlags {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.flags(from_glib_borrow::<_, Paintable>(paintable).unsafe_cast_ref())
        .to_glib()
}

unsafe extern "C" fn paintable_get_intrinsic_width<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> i32 {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.intrinsic_width(from_glib_borrow::<_, Paintable>(paintable).unsafe_cast_ref())
}

unsafe extern "C" fn paintable_get_intrinsic_height<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> i32 {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.intrinsic_height(from_glib_borrow::<_, Paintable>(paintable).unsafe_cast_ref())
}

unsafe extern "C" fn paintable_get_intrinsic_aspect_ratio<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> f64 {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.intrinsic_aspect_ratio(from_glib_borrow::<_, Paintable>(paintable).unsafe_cast_ref())
}

unsafe extern "C" fn paintable_snapshot<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
    snapshotptr: *mut ffi::GdkSnapshot,
    width: f64,
    height: f64,
) {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.snapshot(
        from_glib_borrow::<_, Paintable>(paintable).unsafe_cast_ref(),
        &Snapshot::from_glib_borrow(snapshotptr),
        width,
        height,
    )
}
