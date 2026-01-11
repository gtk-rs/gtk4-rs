// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`SymbolicPaintable`] interface.

use glib::translate::*;

use crate::{SymbolicPaintable, ffi, prelude::*, subclass::prelude::*};

pub trait SymbolicPaintableImpl:
    PaintableImpl + ObjectSubclass<Type: IsA<SymbolicPaintable>>
{
    fn snapshot_symbolic(
        &self,
        snapshot: &gdk::Snapshot,
        width: f64,
        height: f64,
        colors: &[gdk::RGBA],
    ) {
        self.parent_snapshot_symbolic(snapshot, width, height, colors)
    }

    #[cfg(feature = "v4_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_22")))]
    fn snapshot_with_weight(
        &self,
        snapshot: &gdk::Snapshot,
        width: f64,
        height: f64,
        colors: &[gdk::RGBA],
        weight: f64,
    ) {
        self.parent_snapshot_with_weight(snapshot, width, height, colors, weight)
    }
}

pub trait SymbolicPaintableImplExt: SymbolicPaintableImpl {
    fn parent_snapshot_symbolic(
        &self,
        snapshot: &gdk::Snapshot,
        width: f64,
        height: f64,
        colors: &[gdk::RGBA],
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<SymbolicPaintable>()
                as *const ffi::GtkSymbolicPaintableInterface;

            let func = (*parent_iface).snapshot_symbolic.unwrap();
            func(
                self.obj()
                    .unsafe_cast_ref::<SymbolicPaintable>()
                    .to_glib_none()
                    .0,
                snapshot.to_glib_none().0,
                width,
                height,
                colors.as_ptr() as *const gdk::ffi::GdkRGBA,
                colors.len() as _,
            )
        }
    }

    #[cfg(feature = "v4_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_22")))]
    fn parent_snapshot_with_weight(
        &self,
        snapshot: &gdk::Snapshot,
        width: f64,
        height: f64,
        colors: &[gdk::RGBA],
        weight: f64,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<SymbolicPaintable>()
                as *const ffi::GtkSymbolicPaintableInterface;

            let func = (*parent_iface).snapshot_with_weight.unwrap();
            func(
                self.obj()
                    .unsafe_cast_ref::<SymbolicPaintable>()
                    .to_glib_none()
                    .0,
                snapshot.to_glib_none().0,
                width,
                height,
                colors.as_ptr() as *const gdk::ffi::GdkRGBA,
                colors.len() as _,
                weight,
            )
        }
    }
}

impl<T: SymbolicPaintableImpl> SymbolicPaintableImplExt for T {}

unsafe impl<T: SymbolicPaintableImpl> IsImplementable<T> for SymbolicPaintable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        assert_initialized_main_thread!();

        iface.snapshot_symbolic = Some(symbolic_paintable_snapshot_symbolic::<T>);
        #[cfg(feature = "v4_22")]
        {
            iface.snapshot_with_weight = Some(symbolic_paintable_snapshot_with_weight::<T>);
        }
    }
}

unsafe extern "C" fn symbolic_paintable_snapshot_symbolic<T: SymbolicPaintableImpl>(
    paintable: *mut ffi::GtkSymbolicPaintable,
    snapshotptr: *mut gdk::ffi::GdkSnapshot,
    width: f64,
    height: f64,
    colors: *const gdk::ffi::GdkRGBA,
    n_colors: usize,
) {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.imp();

    let snapshot: Borrowed<gdk::Snapshot> = from_glib_borrow(snapshotptr);

    imp.snapshot_symbolic(
        &snapshot,
        width,
        height,
        if n_colors == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(colors as *const gdk::RGBA, n_colors)
        },
    )
}

#[cfg(feature = "v4_22")]
unsafe extern "C" fn symbolic_paintable_snapshot_with_weight<T: SymbolicPaintableImpl>(
    paintable: *mut ffi::GtkSymbolicPaintable,
    snapshotptr: *mut gdk::ffi::GdkSnapshot,
    width: f64,
    height: f64,
    colors: *const gdk::ffi::GdkRGBA,
    n_colors: usize,
    weight: f64,
) {
    let instance = &*(paintable as *mut T::Instance);
    let imp = instance.imp();

    let snapshot: Borrowed<gdk::Snapshot> = from_glib_borrow(snapshotptr);

    imp.snapshot_with_weight(
        &snapshot,
        width,
        height,
        if n_colors == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(colors as *const gdk::RGBA, n_colors)
        },
        weight,
    )
}
