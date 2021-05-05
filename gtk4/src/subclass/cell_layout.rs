// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::{CellArea, CellLayout, CellRenderer, TreeIter, TreeModel};
use glib::translate::*;
use glib::{Cast, IsA, ObjectExt, Quark};
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct CellLayoutDataCallback {
    callback: ffi::GtkCellLayoutDataFunc,
    user_data: glib::ffi::gpointer,
    destroy_notify: glib::ffi::GDestroyNotify,
}

impl CellLayoutDataCallback {
    pub fn call<C: IsA<CellLayout>, R: IsA<CellRenderer>, M: IsA<TreeModel>>(
        &self,
        cell_layout: &C,
        cell_renderer: &R,
        model: &M,
        iter: &TreeIter,
    ) {
        unsafe {
            if let Some(callback) = self.callback {
                callback(
                    cell_layout.as_ref().to_glib_none().0,
                    cell_renderer.as_ref().to_glib_none().0,
                    model.as_ref().to_glib_none().0,
                    mut_override(iter.to_glib_none().0),
                    self.user_data,
                );
            }
        }
    }
}

impl Drop for CellLayoutDataCallback {
    fn drop(&mut self) {
        unsafe {
            if let Some(destroy_notify) = self.destroy_notify {
                destroy_notify(self.user_data);
            }
        }
    }
}

pub trait CellLayoutImpl: ObjectImpl {
    fn add_attribute<R: IsA<CellRenderer>>(
        &self,
        cell_layout: &Self::Type,
        cell: &R,
        attribute: &str,
        column: i32,
    ) {
        self.parent_add_attribute(cell_layout, cell, attribute, column)
    }

    fn clear_attributes<R: IsA<CellRenderer>>(&self, cell_layout: &Self::Type, cell: &R) {
        self.parent_clear_attributes(cell_layout, cell)
    }

    fn cells(&self, cell_layout: &Self::Type) -> Vec<CellRenderer> {
        self.parent_cells(cell_layout)
    }

    fn set_cell_data_func<R: IsA<CellRenderer>>(
        &self,
        cell_layout: &Self::Type,
        cell: &R,
        callback: Option<CellLayoutDataCallback>,
    ) {
        self.parent_set_cell_data_func(cell_layout, cell, callback)
    }

    fn reorder<R: IsA<CellRenderer>>(&self, cell_layout: &Self::Type, cell: &R, position: i32) {
        self.parent_reorder(cell_layout, cell, position)
    }

    fn clear(&self, cell_layout: &Self::Type) {
        self.parent_clear(cell_layout)
    }

    fn pack_start<R: IsA<CellRenderer>>(&self, cell_layout: &Self::Type, cell: &R, expand: bool) {
        self.parent_pack_start(cell_layout, cell, expand)
    }

    fn pack_end<R: IsA<CellRenderer>>(&self, cell_layout: &Self::Type, cell: &R, expand: bool) {
        self.parent_pack_end(cell_layout, cell, expand)
    }

    fn area(&self, cell_layout: &Self::Type) -> Option<CellArea> {
        self.parent_area(cell_layout)
    }
}

pub trait CellLayoutImplExt: ObjectSubclass {
    fn parent_add_attribute<R: IsA<CellRenderer>>(
        &self,
        cell_layout: &Self::Type,
        cell: &R,
        attribute: &str,
        column: i32,
    );
    fn parent_clear_attributes<R: IsA<CellRenderer>>(&self, cell_layout: &Self::Type, cell: &R);
    fn parent_cells(&self, cell_layout: &Self::Type) -> Vec<CellRenderer>;
    fn parent_set_cell_data_func<R: IsA<CellRenderer>>(
        &self,
        cell_layout: &Self::Type,
        cell: &R,
        callback: Option<CellLayoutDataCallback>,
    );
    fn parent_reorder<R: IsA<CellRenderer>>(
        &self,
        cell_layout: &Self::Type,
        cell: &R,
        position: i32,
    );
    fn parent_clear(&self, cell_layout: &Self::Type);
    fn parent_pack_start<R: IsA<CellRenderer>>(
        &self,
        cell_layout: &Self::Type,
        cell: &R,
        expand: bool,
    );
    fn parent_pack_end<R: IsA<CellRenderer>>(
        &self,
        cell_layout: &Self::Type,
        cell: &R,
        expand: bool,
    );
    fn parent_area(&self, cell_layout: &Self::Type) -> Option<CellArea>;
}

impl<O: CellLayoutImpl> CellLayoutImplExt for O {
    fn parent_add_attribute<R: IsA<CellRenderer>>(
        &self,
        cell_layout: &Self::Type,
        cell: &R,
        attribute: &str,
        column: i32,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            if let Some(f) = (*parent_iface).add_attribute {
                f(
                    cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                    attribute.to_glib_none().0,
                    column,
                );
            }
        }
    }

    fn parent_clear_attributes<R: IsA<CellRenderer>>(&self, cell_layout: &Self::Type, cell: &R) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            if let Some(f) = (*parent_iface).clear_attributes {
                f(
                    cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                );
            }
        }
    }

    fn parent_cells(&self, cell_layout: &Self::Type) -> Vec<CellRenderer> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            let f = (*parent_iface)
                .get_cells
                .as_ref()
                .expect("no parent \"get_cells\" implementation");

            let cells = f(cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0);
            FromGlibPtrArrayContainerAsVec::from_glib_container_as_vec(cells)
        }
    }

    fn parent_set_cell_data_func<R: IsA<CellRenderer>>(
        &self,
        cell_layout: &Self::Type,
        cell: &R,
        callback: Option<CellLayoutDataCallback>,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            let f = (*parent_iface)
                .set_cell_data_func
                .as_ref()
                .expect("no parent \"set_cell_data_func\" implementation");

            if let Some(data_cb) = callback {
                f(
                    cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                    data_cb.callback,
                    data_cb.user_data,
                    data_cb.destroy_notify,
                );
            } else {
                f(
                    cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                    None,
                    std::ptr::null_mut(),
                    None,
                );
            }
        }
    }

    fn parent_reorder<R: IsA<CellRenderer>>(
        &self,
        cell_layout: &Self::Type,
        cell: &R,
        position: i32,
    ) {
        {
            unsafe {
                let type_data = Self::type_data();
                let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                    as *const ffi::GtkCellLayoutIface;

                if let Some(f) = (*parent_iface).reorder {
                    f(
                        cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                        cell.as_ref().to_glib_none().0,
                        position,
                    );
                }
            }
        }
    }

    fn parent_clear(&self, cell_layout: &Self::Type) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            if let Some(f) = (*parent_iface).clear {
                f(cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0);
            }
        }
    }

    fn parent_pack_start<R: IsA<CellRenderer>>(
        &self,
        cell_layout: &Self::Type,
        cell: &R,
        expand: bool,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            if let Some(f) = (*parent_iface).pack_start {
                f(
                    cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                    expand.into_glib(),
                );
            }
        }
    }

    fn parent_pack_end<R: IsA<CellRenderer>>(
        &self,
        cell_layout: &Self::Type,
        cell: &R,
        expand: bool,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            if let Some(f) = (*parent_iface).pack_end {
                f(
                    cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                    expand.into_glib(),
                );
            }
        }
    }

    fn parent_area(&self, cell_layout: &Self::Type) -> Option<CellArea> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            (*parent_iface).get_area.map(|f| {
                from_glib_none(f(cell_layout
                    .unsafe_cast_ref::<CellLayout>()
                    .to_glib_none()
                    .0))
            })
        }
    }
}

unsafe impl<T: CellLayoutImpl> IsImplementable<T> for CellLayout {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.get_area = Some(cell_layout_get_area::<T>);
        iface.pack_start = Some(cell_layout_pack_start::<T>);
        iface.pack_end = Some(cell_layout_pack_end::<T>);
        iface.clear = Some(cell_layout_clear::<T>);
        iface.reorder = Some(cell_layout_reorder::<T>);
        iface.add_attribute = Some(cell_layout_add_attribute::<T>);
        iface.clear_attributes = Some(cell_layout_clear_attributes::<T>);
        iface.set_cell_data_func = Some(cell_layout_set_cell_data_func::<T>);
        iface.get_cells = Some(cell_layout_get_cells::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn cell_layout_get_area<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
) -> *mut ffi::GtkCellArea {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.impl_();

    imp.area(from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn cell_layout_pack_start<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    expand: glib::ffi::gboolean,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.impl_();

    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    imp.pack_start(
        from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref(),
        &*cell,
        from_glib(expand),
    )
}

unsafe extern "C" fn cell_layout_pack_end<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    expand: glib::ffi::gboolean,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.impl_();

    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    imp.pack_end(
        from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref(),
        &*cell,
        from_glib(expand),
    )
}

unsafe extern "C" fn cell_layout_clear<T: CellLayoutImpl>(cell_layout: *mut ffi::GtkCellLayout) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.impl_();

    imp.clear(from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref())
}

unsafe extern "C" fn cell_layout_reorder<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    position: i32,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.impl_();

    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    imp.reorder(
        from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref(),
        &*cell,
        position,
    )
}

unsafe extern "C" fn cell_layout_add_attribute<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    attributeptr: *const libc::c_char,
    column: i32,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.impl_();

    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);
    let attribute: Borrowed<glib::GString> = from_glib_borrow(attributeptr);

    imp.add_attribute(
        from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref(),
        &*cell,
        &attribute,
        column,
    )
}

unsafe extern "C" fn cell_layout_clear_attributes<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.impl_();

    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    imp.clear_attributes(
        from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref(),
        &*cell,
    )
}

unsafe extern "C" fn cell_layout_set_cell_data_func<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    callback: ffi::GtkCellLayoutDataFunc,
    user_data: glib::ffi::gpointer,
    destroy_notify: glib::ffi::GDestroyNotify,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.impl_();

    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    let callback = if callback.is_none() {
        None
    } else {
        Some(CellLayoutDataCallback {
            callback,
            user_data,
            destroy_notify,
        })
    };

    imp.set_cell_data_func(
        from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref(),
        &*cell,
        callback,
    )
}

static CELL_LAYOUT_GET_CELLS_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-subclass-cell-layout-get-cells"));

unsafe extern "C" fn cell_layout_get_cells<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
) -> *mut glib::ffi::GList {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.impl_();

    let wrap = from_glib_borrow::<_, CellLayout>(cell_layout);

    let cells = imp.cells(wrap.unsafe_cast_ref());

    let ptr = ToGlibContainerFromSlice::to_glib_none_from_slice(&cells).0;
    wrap.set_qdata(*CELL_LAYOUT_GET_CELLS_QUARK, ptr);
    ptr
}
