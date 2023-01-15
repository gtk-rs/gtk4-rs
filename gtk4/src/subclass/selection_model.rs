// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`SelectionModel`](crate::SelectionModel) interface.

use crate::{prelude::*, subclass::prelude::*, Bitset, SelectionModel};
use glib::translate::*;

pub trait SelectionModelImpl: ListModelImpl {
    #[doc(alias = "get_selection_in_range")]
    fn selection_in_range(&self, position: u32, n_items: u32) -> Bitset {
        self.parent_selection_in_range(position, n_items)
    }

    fn is_selected(&self, position: u32) -> bool {
        self.parent_is_selected(position)
    }

    fn select_all(&self) -> bool {
        self.parent_select_all()
    }

    fn select_item(&self, position: u32, unselect_rest: bool) -> bool {
        self.parent_select_item(position, unselect_rest)
    }

    fn select_range(&self, position: u32, n_items: u32, unselect_rest: bool) -> bool {
        self.parent_select_range(position, n_items, unselect_rest)
    }

    fn set_selection(&self, selected: &Bitset, mask: &Bitset) -> bool {
        self.parent_set_selection(selected, mask)
    }

    fn unselect_all(&self) -> bool {
        self.parent_unselect_all()
    }

    fn unselect_item(&self, position: u32) -> bool {
        self.parent_unselect_item(position)
    }

    fn unselect_range(&self, position: u32, n_items: u32) -> bool {
        self.parent_unselect_range(position, n_items)
    }
}

pub trait SelectionModelImplExt: ObjectSubclass {
    fn parent_selection_in_range(&self, position: u32, n_items: u32) -> Bitset;
    fn parent_is_selected(&self, position: u32) -> bool;
    fn parent_select_all(&self) -> bool;
    fn parent_select_item(&self, position: u32, unselect_rest: bool) -> bool;
    fn parent_select_range(&self, position: u32, n_items: u32, unselect_rest: bool) -> bool;
    fn parent_set_selection(&self, selected: &Bitset, mask: &Bitset) -> bool;
    fn parent_unselect_all(&self) -> bool;
    fn parent_unselect_item(&self, position: u32) -> bool;
    fn parent_unselect_range(&self, position: u32, n_items: u32) -> bool;
}

impl<T: SelectionModelImpl> SelectionModelImplExt for T {
    fn parent_selection_in_range(&self, position: u32, n_items: u32) -> Bitset {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<SelectionModel>()
                as *const ffi::GtkSelectionModelInterface;

            let func = (*parent_iface)
                .get_selection_in_range
                .expect("no parent \"get_selection_in_range\" implementation");

            from_glib_full(func(
                self.obj()
                    .unsafe_cast_ref::<SelectionModel>()
                    .to_glib_none()
                    .0,
                position,
                n_items,
            ))
        }
    }

    fn parent_is_selected(&self, position: u32) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<SelectionModel>()
                as *const ffi::GtkSelectionModelInterface;

            let func = (*parent_iface)
                .is_selected
                .expect("no parent \"is_selected\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<SelectionModel>()
                    .to_glib_none()
                    .0,
                position,
            ))
        }
    }

    fn parent_select_all(&self) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<SelectionModel>()
                as *const ffi::GtkSelectionModelInterface;

            let func = (*parent_iface)
                .select_all
                .expect("no parent \"select_all\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<SelectionModel>()
                    .to_glib_none()
                    .0,
            ))
        }
    }

    fn parent_select_item(&self, position: u32, unselect_rest: bool) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<SelectionModel>()
                as *const ffi::GtkSelectionModelInterface;

            let func = (*parent_iface)
                .select_item
                .expect("no parent \"select_item\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<SelectionModel>()
                    .to_glib_none()
                    .0,
                position,
                unselect_rest.into_glib(),
            ))
        }
    }

    fn parent_select_range(&self, position: u32, n_items: u32, unselect_rest: bool) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<SelectionModel>()
                as *const ffi::GtkSelectionModelInterface;

            let func = (*parent_iface)
                .select_range
                .expect("no parent \"select_range\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<SelectionModel>()
                    .to_glib_none()
                    .0,
                position,
                n_items,
                unselect_rest.into_glib(),
            ))
        }
    }

    fn parent_set_selection(&self, selected: &Bitset, mask: &Bitset) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<SelectionModel>()
                as *const ffi::GtkSelectionModelInterface;

            let func = (*parent_iface)
                .set_selection
                .expect("no parent \"set_selection\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<SelectionModel>()
                    .to_glib_none()
                    .0,
                selected.to_glib_none().0,
                mask.to_glib_none().0,
            ))
        }
    }

    fn parent_unselect_all(&self) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<SelectionModel>()
                as *const ffi::GtkSelectionModelInterface;

            let func = (*parent_iface)
                .unselect_all
                .expect("no parent \"unselect_all\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<SelectionModel>()
                    .to_glib_none()
                    .0,
            ))
        }
    }

    fn parent_unselect_item(&self, position: u32) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<SelectionModel>()
                as *const ffi::GtkSelectionModelInterface;

            let func = (*parent_iface)
                .unselect_item
                .expect("no parent \"unselect_item\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<SelectionModel>()
                    .to_glib_none()
                    .0,
                position,
            ))
        }
    }

    fn parent_unselect_range(&self, position: u32, n_items: u32) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<SelectionModel>()
                as *const ffi::GtkSelectionModelInterface;

            let func = (*parent_iface)
                .unselect_range
                .expect("no parent \"unselect_range\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<SelectionModel>()
                    .to_glib_none()
                    .0,
                position,
                n_items,
            ))
        }
    }
}

unsafe impl<T: SelectionModelImpl> IsImplementable<T> for SelectionModel {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        assert_initialized_main_thread!();

        iface.get_selection_in_range = Some(model_get_selection_in_range::<T>);
        iface.is_selected = Some(model_is_selected::<T>);
        iface.select_all = Some(model_select_all::<T>);
        iface.select_item = Some(model_select_item::<T>);
        iface.select_range = Some(model_select_range::<T>);
        iface.set_selection = Some(model_set_selection::<T>);
        iface.unselect_all = Some(model_unselect_all::<T>);
        iface.unselect_item = Some(model_unselect_item::<T>);
        iface.unselect_range = Some(model_unselect_range::<T>);
    }
}

unsafe extern "C" fn model_get_selection_in_range<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    position: u32,
    n_items: u32,
) -> *mut ffi::GtkBitset {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.imp();

    imp.selection_in_range(position, n_items).into_glib_ptr()
}

unsafe extern "C" fn model_is_selected<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    position: u32,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.imp();

    imp.is_selected(position).into_glib()
}

unsafe extern "C" fn model_select_all<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.imp();

    imp.select_all().into_glib()
}

unsafe extern "C" fn model_select_item<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    position: u32,
    unselect_rest: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.imp();

    imp.select_item(position, from_glib(unselect_rest))
        .into_glib()
}

unsafe extern "C" fn model_select_range<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    position: u32,
    n_items: u32,
    unselect_rest: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.imp();

    imp.select_range(position, n_items, from_glib(unselect_rest))
        .into_glib()
}

unsafe extern "C" fn model_set_selection<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    selected_ptr: *mut ffi::GtkBitset,
    mask_ptr: *mut ffi::GtkBitset,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.imp();

    let selected = from_glib_borrow(selected_ptr);
    let mask = from_glib_borrow(mask_ptr);

    imp.set_selection(&selected, &mask).into_glib()
}

unsafe extern "C" fn model_unselect_all<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.imp();

    imp.unselect_all().into_glib()
}

unsafe extern "C" fn model_unselect_item<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    position: u32,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.imp();

    imp.unselect_item(position).into_glib()
}

unsafe extern "C" fn model_unselect_range<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    position: u32,
    n_items: u32,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.imp();

    imp.unselect_range(position, n_items).into_glib()
}
