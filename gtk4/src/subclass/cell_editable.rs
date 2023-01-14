// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`CellEditable`](crate::CellEditable) interface.

use crate::{prelude::*, subclass::prelude::*, CellEditable};
use glib::translate::*;

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait CellEditableImpl: ObjectImpl {
    fn editing_done(&self) {
        self.parent_editing_done()
    }

    fn remove_widget(&self) {
        self.parent_remove_widget()
    }

    fn start_editing(&self, event: Option<&gdk::Event>) {
        self.parent_start_editing(event)
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait CellEditableImplExt: ObjectSubclass {
    fn parent_editing_done(&self);
    fn parent_remove_widget(&self);
    fn parent_start_editing(&self, event: Option<&gdk::Event>);
}

impl<O: CellEditableImpl> CellEditableImplExt for O {
    fn parent_editing_done(&self) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellEditable>()
                as *const ffi::GtkCellEditableIface;

            if let Some(f) = (*parent_iface).editing_done {
                f(self
                    .obj()
                    .unsafe_cast_ref::<CellEditable>()
                    .to_glib_none()
                    .0);
            }
        }
    }

    fn parent_remove_widget(&self) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellEditable>()
                as *const ffi::GtkCellEditableIface;

            if let Some(f) = (*parent_iface).remove_widget {
                f(self
                    .obj()
                    .unsafe_cast_ref::<CellEditable>()
                    .to_glib_none()
                    .0);
            }
        }
    }

    fn parent_start_editing(&self, event: Option<&gdk::Event>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellEditable>()
                as *const ffi::GtkCellEditableIface;

            if let Some(f) = (*parent_iface).start_editing {
                f(
                    self.obj()
                        .unsafe_cast_ref::<CellEditable>()
                        .to_glib_none()
                        .0,
                    event.to_glib_none().0,
                );
            }
        }
    }
}

unsafe impl<T: CellEditableImpl> IsImplementable<T> for CellEditable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        assert_initialized_main_thread!();

        iface.editing_done = Some(cell_editable_editing_done::<T>);
        iface.remove_widget = Some(cell_editable_remove_widget::<T>);
        iface.start_editing = Some(cell_editable_start_editing::<T>);
    }
}

unsafe extern "C" fn cell_editable_editing_done<T: CellEditableImpl>(
    cell_editable: *mut ffi::GtkCellEditable,
) {
    let instance = &*(cell_editable as *mut T::Instance);
    let imp = instance.imp();

    imp.editing_done()
}

unsafe extern "C" fn cell_editable_remove_widget<T: CellEditableImpl>(
    cell_editable: *mut ffi::GtkCellEditable,
) {
    let instance = &*(cell_editable as *mut T::Instance);
    let imp = instance.imp();

    imp.remove_widget()
}

unsafe extern "C" fn cell_editable_start_editing<T: CellEditableImpl>(
    cell_editable: *mut ffi::GtkCellEditable,
    eventptr: *mut gdk::ffi::GdkEvent,
) {
    let instance = &*(cell_editable as *mut T::Instance);
    let imp = instance.imp();
    let event: Borrowed<Option<gdk::Event>> = from_glib_borrow(eventptr);
    imp.start_editing(event.as_ref().as_ref())
}
