// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::widget::WidgetImpl;
use crate::{Border, Range, ScrollType, Widget};

pub trait RangeImpl: RangeImplExt + WidgetImpl {
    fn adjust_bounds(&self, range: &Self::Type, new_value: f64) {
        self.parent_adjust_bounds(range, new_value)
    }

    fn change_value(&self, range: &Self::Type, scroll_type: ScrollType, new_value: f64) -> bool {
        self.parent_change_value(range, scroll_type, new_value)
    }

    fn get_range_border(&self, range: &Self::Type) -> Border {
        self.parent_get_range_border(range)
    }

    fn move_slider(&self, range: &Self::Type, scroll_type: ScrollType) {
        self.parent_move_slider(range, scroll_type)
    }

    fn value_changed(&self, range: &Self::Type) {
        self.parent_value_changed(range)
    }
}

pub trait RangeImplExt: ObjectSubclass {
    fn parent_adjust_bounds(&self, range: &Self::Type, new_value: f64);
    fn parent_change_value(
        &self,
        range: &Self::Type,
        scroll_type: ScrollType,
        new_value: f64,
    ) -> bool;
    fn parent_get_range_border(&self, range: &Self::Type) -> Border;
    fn parent_move_slider(&self, range: &Self::Type, scroll_type: ScrollType);
    fn parent_value_changed(&self, range: &Self::Type);
}

impl<T: RangeImpl> RangeImplExt for T {
    fn parent_adjust_bounds(&self, range: &Self::Type, new_value: f64) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkRangeClass;
            if let Some(f) = (*parent_class).adjust_bounds {
                f(range.unsafe_cast_ref::<Range>().to_glib_none().0, new_value)
            }
        }
    }

    fn parent_change_value(
        &self,
        range: &Self::Type,
        scroll_type: ScrollType,
        new_value: f64,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkRangeClass;
            let f = (*parent_class)
                .change_value
                .expect("No parent class impl for \"change_value\"");
            from_glib(f(
                range.unsafe_cast_ref::<Range>().to_glib_none().0,
                scroll_type.to_glib(),
                new_value,
            ))
        }
    }

    fn parent_get_range_border(&self, range: &Self::Type) -> Border {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkRangeClass;
            let mut border = Border::default();
            if let Some(f) = (*parent_class).get_range_border {
                f(
                    range.unsafe_cast_ref::<Range>().to_glib_none().0,
                    border.to_glib_none_mut().0,
                );
            }
            border
        }
    }

    fn parent_move_slider(&self, range: &Self::Type, scroll_type: ScrollType) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkRangeClass;
            if let Some(f) = (*parent_class).move_slider {
                f(
                    range.unsafe_cast_ref::<Range>().to_glib_none().0,
                    scroll_type.to_glib(),
                )
            }
        }
    }

    fn parent_value_changed(&self, range: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkRangeClass;
            if let Some(f) = (*parent_class).value_changed {
                f(range.unsafe_cast_ref::<Range>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: RangeImpl> IsSubclassable<T> for Range {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.adjust_bounds = Some(range_adjust_bounds::<T>);
        klass.change_value = Some(range_change_value::<T>);
        klass.get_range_border = Some(range_get_range_border::<T>);
        klass.move_slider = Some(range_move_slider::<T>);
        klass.value_changed = Some(range_value_changed::<T>);
    }
}

unsafe extern "C" fn range_adjust_bounds<T: RangeImpl>(ptr: *mut ffi::GtkRange, new_value: f64) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Range> = from_glib_borrow(ptr);

    imp.adjust_bounds(wrap.unsafe_cast_ref(), new_value)
}

unsafe extern "C" fn range_change_value<T: RangeImpl>(
    ptr: *mut ffi::GtkRange,
    scroll_type: ffi::GtkScrollType,
    new_value: f64,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Range> = from_glib_borrow(ptr);

    imp.change_value(wrap.unsafe_cast_ref(), from_glib(scroll_type), new_value)
        .to_glib()
}

unsafe extern "C" fn range_get_range_border<T: RangeImpl>(
    ptr: *mut ffi::GtkRange,
    borderptr: *mut ffi::GtkBorder,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Range> = from_glib_borrow(ptr);

    let border = imp.get_range_border(wrap.unsafe_cast_ref());
    *borderptr = *border.to_glib_none().0;
}

unsafe extern "C" fn range_move_slider<T: RangeImpl>(
    ptr: *mut ffi::GtkRange,
    scroll_type: ffi::GtkScrollType,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Range> = from_glib_borrow(ptr);

    imp.move_slider(wrap.unsafe_cast_ref(), from_glib(scroll_type))
}

unsafe extern "C" fn range_value_changed<T: RangeImpl>(ptr: *mut ffi::GtkRange) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Range> = from_glib_borrow(ptr);

    imp.value_changed(wrap.unsafe_cast_ref())
}
