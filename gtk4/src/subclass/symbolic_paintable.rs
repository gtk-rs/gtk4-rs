// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`SymbolicPaintable`](crate::SymbolicPaintable) interface.

use crate::{prelude::*, subclass::prelude::*, SymbolicPaintable};
use glib::translate::*;

pub trait SymbolicPaintableImpl: PaintableImpl {
    fn snapshot_symbolic(
        &self,
        snapshot: &gdk::Snapshot,
        width: f64,
        height: f64,
        colors: &[gdk::RGBA],
    ) {
        self.parent_snapshot_symbolic(snapshot, width, height, colors)
    }
}

pub trait SymbolicPaintableImplExt: ObjectSubclass {
    fn parent_snapshot_symbolic(
        &self,
        _snapshot: &gdk::Snapshot,
        _width: f64,
        _height: f64,
        _colors: &[gdk::RGBA],
    );
}

impl<T: SymbolicPaintableImpl> SymbolicPaintableImplExt for T {
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
                colors.to_glib_none().0,
                colors.len() as _,
            )
        }
    }
}

unsafe impl<T: SymbolicPaintableImpl> IsImplementable<T> for SymbolicPaintable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        assert_initialized_main_thread!();

        iface.snapshot_symbolic = Some(symbolic_paintable_snapshot_symbolic::<T>);
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
