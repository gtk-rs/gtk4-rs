// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Bitset;
use crate::SelectionModel;
use gio::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait SelectionModelImpl: ObjectImpl + ListModelImpl {
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

    fn is_selected(&self, model: &Self::Type, position: u32) -> bool;
    fn select_all(&self, model: &Self::Type) -> bool;
    fn select_item(&self, model: &Self::Type, position: u32, unselect_rest: bool) -> bool;
    fn select_range(
        &self,
        model: &Self::Type,
        position: u32,
        n_items: u32,
        unselect_rest: bool,
    ) -> bool;
    fn set_selection(&self, model: &Self::Type, selected: &Bitset, mask: &Bitset) -> bool;
    fn unselect_all(&self, model: &Self::Type) -> bool;
    fn unselect_item(&self, model: &Self::Type, position: u32) -> bool;
    fn unselect_range(&self, model: &Self::Type, position: u32, n_items: u32) -> bool;
}

unsafe impl<T: SelectionModelImpl> IsImplementable<T> for SelectionModel {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let model_iface = &mut *(iface as *mut ffi::GtkSelectionModelInterface);
        model_iface.get_selection_in_range = Some(model_get_selection_in_range::<T>);
        model_iface.is_selected = Some(model_is_selected::<T>);
        model_iface.select_all = Some(model_select_all::<T>);
        model_iface.select_item = Some(model_select_item::<T>);
        model_iface.select_range = Some(model_select_range::<T>);
        model_iface.set_selection = Some(model_set_selection::<T>);
        model_iface.unselect_all = Some(model_unselect_all::<T>);
        model_iface.unselect_item = Some(model_unselect_item::<T>);
        model_iface.unselect_range = Some(model_unselect_range::<T>);
    }
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
