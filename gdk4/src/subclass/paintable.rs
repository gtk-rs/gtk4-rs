// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Paintable, PaintableFlags, Snapshot};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait PaintableImpl: ObjectImpl {
    fn get_current_image(&self, paintable: &Self::Type) -> Paintable {
        unsafe {
            let type_ = ffi::gdk_paintable_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GdkPaintableInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).get_current_image.as_ref().unwrap())(
                paintable.unsafe_cast_ref::<Paintable>().to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib_full(ret)
        }
    }
    fn get_flags(&self, paintable: &Self::Type) -> PaintableFlags {
        unsafe {
            let type_ = ffi::gdk_paintable_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GdkPaintableInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).get_flags.as_ref().unwrap())(
                paintable.unsafe_cast_ref::<Paintable>().to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib(ret)
        }
    }
    fn get_intrinsic_width(&self, paintable: &Self::Type) -> i32 {
        unsafe {
            let type_ = ffi::gdk_paintable_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GdkPaintableInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).get_intrinsic_width.as_ref().unwrap())(
                paintable.unsafe_cast_ref::<Paintable>().to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            ret
        }
    }
    fn get_intrinsic_height(&self, paintable: &Self::Type) -> i32 {
        unsafe {
            let type_ = ffi::gdk_paintable_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GdkPaintableInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).get_intrinsic_height.as_ref().unwrap())(
                paintable.unsafe_cast_ref::<Paintable>().to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            ret
        }
    }
    fn get_intrinsic_aspect_ratio(&self, paintable: &Self::Type) -> f64 {
        {
            unsafe {
                let type_ = ffi::gdk_paintable_get_type();
                let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                    as *mut ffi::GdkPaintableInterface;
                assert!(!iface.is_null());

                let ret = ((*iface).get_intrinsic_aspect_ratio.as_ref().unwrap())(
                    paintable.unsafe_cast_ref::<Paintable>().to_glib_none().0,
                );

                glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

                ret
            }
        }
    }
    fn snapshot(&self, paintable: &Self::Type, snapshot: &Snapshot, width: f64, height: f64);
}

unsafe impl<T: PaintableImpl> IsImplementable<T> for Paintable {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let paintable_iface = &mut *(iface as *mut ffi::GdkPaintableInterface);

        paintable_iface.get_current_image = Some(paintable_get_current_image::<T>);
        paintable_iface.get_flags = Some(paintable_get_flags::<T>);
        paintable_iface.get_intrinsic_width = Some(paintable_get_intrinsic_width::<T>);
        paintable_iface.get_intrinsic_height = Some(paintable_get_intrinsic_height::<T>);
        paintable_iface.get_intrinsic_aspect_ratio =
            Some(paintable_get_intrinsic_aspect_ratio::<T>);
        paintable_iface.snapshot = Some(paintable_snapshot::<T>);
    }
}

unsafe extern "C" fn paintable_get_current_image<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> *mut ffi::GdkPaintable {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_current_image(from_glib_borrow::<_, Paintable>(paintable).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn paintable_get_flags<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> ffi::GdkPaintableFlags {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_flags(from_glib_borrow::<_, Paintable>(paintable).unsafe_cast_ref())
        .to_glib()
}

unsafe extern "C" fn paintable_get_intrinsic_width<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> i32 {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_intrinsic_width(from_glib_borrow::<_, Paintable>(paintable).unsafe_cast_ref())
}

unsafe extern "C" fn paintable_get_intrinsic_height<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> i32 {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_intrinsic_height(from_glib_borrow::<_, Paintable>(paintable).unsafe_cast_ref())
}

unsafe extern "C" fn paintable_get_intrinsic_aspect_ratio<T: PaintableImpl>(
    paintable: *mut ffi::GdkPaintable,
) -> f64 {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_intrinsic_aspect_ratio(from_glib_borrow::<_, Paintable>(paintable).unsafe_cast_ref())
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
