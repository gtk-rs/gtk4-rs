// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::CellEditable;
use glib::translate::*;
use glib::Cast;

pub trait CellEditableImpl: ObjectImpl {
    fn editing_done(&self, cell_editable: &Self::Type) {
        self.parent_editing_done(cell_editable)
    }

    fn remove_widget(&self, cell_editable: &Self::Type) {
        self.parent_remove_widget(cell_editable)
    }

    fn start_editing(&self, cell_editable: &Self::Type, event: Option<&gdk::Event>) {
        self.parent_start_editing(cell_editable, event)
    }
}

pub trait CellEditableImplExt: ObjectSubclass {
    fn parent_editing_done(&self, cell_editable: &Self::Type);
    fn parent_remove_widget(&self, cell_editable: &Self::Type);
    fn parent_start_editing(&self, cell_editable: &Self::Type, event: Option<&gdk::Event>);
}

impl<O: CellEditableImpl> CellEditableImplExt for O {
    fn parent_editing_done(&self, cell_editable: &Self::Type) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellEditable>()
                as *const ffi::GtkCellEditableIface;

            if let Some(f) = (*parent_iface).editing_done {
                f(cell_editable
                    .unsafe_cast_ref::<CellEditable>()
                    .to_glib_none()
                    .0);
            }
        }
    }

    fn parent_remove_widget(&self, cell_editable: &Self::Type) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellEditable>()
                as *const ffi::GtkCellEditableIface;

            if let Some(f) = (*parent_iface).remove_widget {
                f(cell_editable
                    .unsafe_cast_ref::<CellEditable>()
                    .to_glib_none()
                    .0);
            }
        }
    }

    fn parent_start_editing(&self, cell_editable: &Self::Type, event: Option<&gdk::Event>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<CellEditable>()
                as *const ffi::GtkCellEditableIface;

            if let Some(f) = (*parent_iface).start_editing {
                f(
                    cell_editable
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

        iface.editing_done = Some(cell_editable_editing_done::<T>);
        iface.remove_widget = Some(cell_editable_remove_widget::<T>);
        iface.start_editing = Some(cell_editable_start_editing::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn cell_editable_editing_done<T: CellEditableImpl>(
    cell_editable: *mut ffi::GtkCellEditable,
) {
    let instance = &*(cell_editable as *mut T::Instance);
    let imp = instance.impl_();

    imp.editing_done(from_glib_borrow::<_, CellEditable>(cell_editable).unsafe_cast_ref())
}

unsafe extern "C" fn cell_editable_remove_widget<T: CellEditableImpl>(
    cell_editable: *mut ffi::GtkCellEditable,
) {
    let instance = &*(cell_editable as *mut T::Instance);
    let imp = instance.impl_();

    imp.remove_widget(from_glib_borrow::<_, CellEditable>(cell_editable).unsafe_cast_ref())
}

unsafe extern "C" fn cell_editable_start_editing<T: CellEditableImpl>(
    cell_editable: *mut ffi::GtkCellEditable,
    eventptr: *mut gdk::ffi::GdkEvent,
) {
    let instance = &*(cell_editable as *mut T::Instance);
    let imp = instance.impl_();
    let event: Borrowed<Option<gdk::Event>> = from_glib_borrow(eventptr);
    imp.start_editing(
        from_glib_borrow::<_, CellEditable>(cell_editable).unsafe_cast_ref(),
        event.as_ref().as_ref(),
    )
}
