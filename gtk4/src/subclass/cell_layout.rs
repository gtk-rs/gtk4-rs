// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`CellLayout`](crate::CellLayout) interface.

use crate::{
    prelude::*, subclass::prelude::*, CellArea, CellLayout, CellRenderer, TreeIter, TreeModel,
};
use glib::{translate::*, Quark};
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
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if let Some(destroy_notify) = self.destroy_notify {
                destroy_notify(self.user_data);
            }
        }
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait CellLayoutImpl: ObjectImpl {
    fn add_attribute<R: IsA<CellRenderer>>(&self, cell: &R, attribute: &str, column: i32) {
        self.parent_add_attribute(cell, attribute, column)
    }

    fn clear_attributes<R: IsA<CellRenderer>>(&self, cell: &R) {
        self.parent_clear_attributes(cell)
    }

    fn cells(&self) -> Vec<CellRenderer> {
        self.parent_cells()
    }

    fn set_cell_data_func<R: IsA<CellRenderer>>(
        &self,

        cell: &R,
        callback: Option<CellLayoutDataCallback>,
    ) {
        self.parent_set_cell_data_func(cell, callback)
    }

    fn reorder<R: IsA<CellRenderer>>(&self, cell: &R, position: i32) {
        self.parent_reorder(cell, position)
    }

    fn clear(&self) {
        self.parent_clear()
    }

    fn pack_start<R: IsA<CellRenderer>>(&self, cell: &R, expand: bool) {
        self.parent_pack_start(cell, expand)
    }

    fn pack_end<R: IsA<CellRenderer>>(&self, cell: &R, expand: bool) {
        self.parent_pack_end(cell, expand)
    }

    fn area(&self) -> Option<CellArea> {
        self.parent_area()
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait CellLayoutImplExt: ObjectSubclass {
    fn parent_add_attribute<R: IsA<CellRenderer>>(&self, cell: &R, attribute: &str, column: i32);
    fn parent_clear_attributes<R: IsA<CellRenderer>>(&self, cell: &R);
    fn parent_cells(&self) -> Vec<CellRenderer>;
    fn parent_set_cell_data_func<R: IsA<CellRenderer>>(
        &self,

        cell: &R,
        callback: Option<CellLayoutDataCallback>,
    );
    fn parent_reorder<R: IsA<CellRenderer>>(&self, cell: &R, position: i32);
    fn parent_clear(&self);
    fn parent_pack_start<R: IsA<CellRenderer>>(&self, cell: &R, expand: bool);
    fn parent_pack_end<R: IsA<CellRenderer>>(&self, cell: &R, expand: bool);
    fn parent_area(&self) -> Option<CellArea>;
}

impl<O: CellLayoutImpl> CellLayoutImplExt for O {
    fn parent_add_attribute<R: IsA<CellRenderer>>(&self, cell: &R, attribute: &str, column: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            if let Some(f) = (*parent_iface).add_attribute {
                f(
                    self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                    attribute.to_glib_none().0,
                    column,
                );
            }
        }
    }

    fn parent_clear_attributes<R: IsA<CellRenderer>>(&self, cell: &R) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            if let Some(f) = (*parent_iface).clear_attributes {
                f(
                    self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                );
            }
        }
    }

    fn parent_cells(&self) -> Vec<CellRenderer> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            let f = (*parent_iface)
                .get_cells
                .as_ref()
                .expect("no parent \"get_cells\" implementation");

            let cells = f(self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0);
            FromGlibPtrArrayContainerAsVec::from_glib_container_as_vec(cells)
        }
    }

    fn parent_set_cell_data_func<R: IsA<CellRenderer>>(
        &self,

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
                    self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                    data_cb.callback,
                    data_cb.user_data,
                    data_cb.destroy_notify,
                );
            } else {
                f(
                    self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                    None,
                    std::ptr::null_mut(),
                    None,
                );
            }
        }
    }

    fn parent_reorder<R: IsA<CellRenderer>>(&self, cell: &R, position: i32) {
        {
            unsafe {
                let type_data = Self::type_data();
                let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                    as *const ffi::GtkCellLayoutIface;

                if let Some(f) = (*parent_iface).reorder {
                    f(
                        self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                        cell.as_ref().to_glib_none().0,
                        position,
                    );
                }
            }
        }
    }

    fn parent_clear(&self) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            if let Some(f) = (*parent_iface).clear {
                f(self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0);
            }
        }
    }

    fn parent_pack_start<R: IsA<CellRenderer>>(&self, cell: &R, expand: bool) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            if let Some(f) = (*parent_iface).pack_start {
                f(
                    self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                    expand.into_glib(),
                );
            }
        }
    }

    fn parent_pack_end<R: IsA<CellRenderer>>(&self, cell: &R, expand: bool) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            if let Some(f) = (*parent_iface).pack_end {
                f(
                    self.obj().unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.as_ref().to_glib_none().0,
                    expand.into_glib(),
                );
            }
        }
    }

    fn parent_area(&self) -> Option<CellArea> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellLayout>()
                as *const ffi::GtkCellLayoutIface;

            (*parent_iface).get_area.map(|f| {
                from_glib_none(f(self
                    .obj()
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

        assert_initialized_main_thread!();

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
}

unsafe extern "C" fn cell_layout_get_area<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
) -> *mut ffi::GtkCellArea {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.imp();

    imp.area().into_glib_ptr()
}

unsafe extern "C" fn cell_layout_pack_start<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    expand: glib::ffi::gboolean,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.imp();
    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    imp.pack_start(&*cell, from_glib(expand))
}

unsafe extern "C" fn cell_layout_pack_end<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    expand: glib::ffi::gboolean,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.imp();
    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    imp.pack_end(&*cell, from_glib(expand))
}

unsafe extern "C" fn cell_layout_clear<T: CellLayoutImpl>(cell_layout: *mut ffi::GtkCellLayout) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.imp();

    imp.clear()
}

unsafe extern "C" fn cell_layout_reorder<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    position: i32,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.imp();
    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    imp.reorder(&*cell, position)
}

unsafe extern "C" fn cell_layout_add_attribute<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    attributeptr: *const libc::c_char,
    column: i32,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.imp();
    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);
    let attribute: Borrowed<glib::GString> = from_glib_borrow(attributeptr);

    imp.add_attribute(&*cell, &attribute, column)
}

unsafe extern "C" fn cell_layout_clear_attributes<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.imp();
    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    imp.clear_attributes(&*cell)
}

unsafe extern "C" fn cell_layout_set_cell_data_func<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    callback: ffi::GtkCellLayoutDataFunc,
    user_data: glib::ffi::gpointer,
    destroy_notify: glib::ffi::GDestroyNotify,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.imp();

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

    imp.set_cell_data_func(&*cell, callback)
}

static CELL_LAYOUT_GET_CELLS_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_str("gtk-rs-subclass-cell-layout-get-cells"));

unsafe extern "C" fn cell_layout_get_cells<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
) -> *mut glib::ffi::GList {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.imp();

    let cells = imp.cells();

    // transfer container: list owned by the caller by not the actual content
    // so we need to keep the cells around and return a ptr of the list
    imp.obj()
        .set_qdata(*CELL_LAYOUT_GET_CELLS_QUARK, cells.clone());
    cells.to_glib_container().0
}
