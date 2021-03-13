// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Bitset, SelectionModel};
use gio::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait SelectionModelImpl: ListModelImpl {
    fn get_selection_in_range(&self, model: &Self::Type, position: u32, n_items: u32) -> Bitset {
        unsafe {
            let type_ = ffi::gtk_selection_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkSelectionModelInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).get_selection_in_range.as_ref().unwrap())(
                model.unsafe_cast_ref::<SelectionModel>().to_glib_none().0,
                position,
                n_items,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib_full(ret)
        }
    }

    fn is_selected(&self, model: &Self::Type, position: u32) -> bool {
        unsafe {
            let type_ = ffi::gtk_selection_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkSelectionModelInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).is_selected.as_ref().unwrap())(
                model.unsafe_cast_ref::<SelectionModel>().to_glib_none().0,
                position,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib(ret)
        }
    }

    fn select_all(&self, model: &Self::Type) -> bool {
        unsafe {
            let type_ = ffi::gtk_selection_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkSelectionModelInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).select_all.as_ref().unwrap())(
                model.unsafe_cast_ref::<SelectionModel>().to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib(ret)
        }
    }

    fn select_item(&self, model: &Self::Type, position: u32, unselect_rest: bool) -> bool {
        unsafe {
            let type_ = ffi::gtk_selection_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkSelectionModelInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).select_item.as_ref().unwrap())(
                model.unsafe_cast_ref::<SelectionModel>().to_glib_none().0,
                position,
                unselect_rest.to_glib(),
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib(ret)
        }
    }

    fn select_range(
        &self,
        model: &Self::Type,
        position: u32,
        n_items: u32,
        unselect_rest: bool,
    ) -> bool {
        unsafe {
            let type_ = ffi::gtk_selection_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkSelectionModelInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).select_range.as_ref().unwrap())(
                model.unsafe_cast_ref::<SelectionModel>().to_glib_none().0,
                position,
                n_items,
                unselect_rest.to_glib(),
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib(ret)
        }
    }

    fn set_selection(&self, model: &Self::Type, selected: &Bitset, mask: &Bitset) -> bool {
        unsafe {
            let type_ = ffi::gtk_selection_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkSelectionModelInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).set_selection.as_ref().unwrap())(
                model.unsafe_cast_ref::<SelectionModel>().to_glib_none().0,
                selected.to_glib_none().0,
                mask.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib(ret)
        }
    }

    fn unselect_all(&self, model: &Self::Type) -> bool {
        unsafe {
            let type_ = ffi::gtk_selection_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkSelectionModelInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).unselect_all.as_ref().unwrap())(
                model.unsafe_cast_ref::<SelectionModel>().to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib(ret)
        }
    }

    fn unselect_item(&self, model: &Self::Type, position: u32) -> bool {
        unsafe {
            let type_ = ffi::gtk_selection_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkSelectionModelInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).unselect_item.as_ref().unwrap())(
                model.unsafe_cast_ref::<SelectionModel>().to_glib_none().0,
                position,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib(ret)
        }
    }

    fn unselect_range(&self, model: &Self::Type, position: u32, n_items: u32) -> bool {
        unsafe {
            let type_ = ffi::gtk_selection_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkSelectionModelInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).unselect_range.as_ref().unwrap())(
                model.unsafe_cast_ref::<SelectionModel>().to_glib_none().0,
                position,
                n_items,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib(ret)
        }
    }
}

unsafe impl<T: SelectionModelImpl> IsImplementable<T> for SelectionModel {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

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

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn model_get_selection_in_range<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    position: u32,
    n_items: u32,
) -> *mut ffi::GtkBitset {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_selection_in_range(
        from_glib_borrow::<_, SelectionModel>(model).unsafe_cast_ref(),
        position,
        n_items,
    )
    .to_glib_full()
}

unsafe extern "C" fn model_is_selected<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    position: u32,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.get_impl();

    imp.is_selected(
        from_glib_borrow::<_, SelectionModel>(model).unsafe_cast_ref(),
        position,
    )
    .to_glib()
}

unsafe extern "C" fn model_select_all<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.get_impl();

    imp.select_all(from_glib_borrow::<_, SelectionModel>(model).unsafe_cast_ref())
        .to_glib()
}

unsafe extern "C" fn model_select_item<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    position: u32,
    unselect_rest: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.get_impl();

    imp.select_item(
        from_glib_borrow::<_, SelectionModel>(model).unsafe_cast_ref(),
        position,
        from_glib(unselect_rest),
    )
    .to_glib()
}

unsafe extern "C" fn model_select_range<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    position: u32,
    n_items: u32,
    unselect_rest: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.get_impl();

    imp.select_range(
        from_glib_borrow::<_, SelectionModel>(model).unsafe_cast_ref(),
        position,
        n_items,
        from_glib(unselect_rest),
    )
    .to_glib()
}

unsafe extern "C" fn model_set_selection<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    selected_ptr: *mut ffi::GtkBitset,
    mask_ptr: *mut ffi::GtkBitset,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.get_impl();

    let wrap: Borrowed<SelectionModel> = from_glib_borrow(model);
    let selected = from_glib_borrow(selected_ptr);
    let mask = from_glib_borrow(mask_ptr);

    imp.set_selection(wrap.unsafe_cast_ref(), &selected, &mask)
        .to_glib()
}

unsafe extern "C" fn model_unselect_all<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.get_impl();

    imp.unselect_all(from_glib_borrow::<_, SelectionModel>(model).unsafe_cast_ref())
        .to_glib()
}

unsafe extern "C" fn model_unselect_item<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    position: u32,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.get_impl();

    imp.unselect_item(
        from_glib_borrow::<_, SelectionModel>(model).unsafe_cast_ref(),
        position,
    )
    .to_glib()
}

unsafe extern "C" fn model_unselect_range<T: SelectionModelImpl>(
    model: *mut ffi::GtkSelectionModel,
    position: u32,
    n_items: u32,
) -> glib::ffi::gboolean {
    let instance = &*(model as *mut T::Instance);
    let imp = instance.get_impl();

    imp.unselect_range(
        from_glib_borrow::<_, SelectionModel>(model).unsafe_cast_ref(),
        position,
        n_items,
    )
    .to_glib()
}
