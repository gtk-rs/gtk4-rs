use gtk_sys;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use crate::TextDirection;
use glib::Object;
use Widget;

pub trait WidgetImpl: WidgetImplExt + ObjectImpl {
    fn compute_expand(&self, widget: &Self::Type, hexpand_p: &mut bool, vexpand_p: &mut bool) {
        self.parent_compute_expand(widget, hexpand_p, vexpand_p)
    }

    fn direction_changed(&self, widget: &Self::Type, previous_direction: TextDirection) {
        self.parent_direction_changed(widget, previous_direction)
    }
}

pub trait WidgetImplExt: ObjectSubclass {
    fn parent_compute_expand(
        &self,
        widget: &Self::Type,
        hexpand_p: &mut bool,
        vexpand_p: &mut bool,
    );
    fn parent_direction_changed(&self, widget: &Self::Type, previous_direction: TextDirection);
}

impl<T: WidgetImpl> WidgetImplExt for T {
    fn parent_compute_expand(
        &self,
        widget: &Self::Type,
        hexpand_p: &mut bool,
        vexpand_p: &mut bool,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .compute_expand
                .expect("No parent class impl for \"compute_expand\"");
            let mut h: i32 = hexpand_p.to_glib();
            let mut v: i32 = vexpand_p.to_glib();
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                &mut h,
                &mut v,
            );
            *hexpand_p = from_glib(h);
            *vexpand_p = from_glib(v);
        }
    }

    fn parent_direction_changed(&self, widget: &Self::Type, previous_direction: TextDirection) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .direction_changed
                .expect("No parent class impl for \"direction_changed\"");
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                previous_direction.to_glib(),
            )
        }
    }
}

unsafe impl<T: WidgetImpl> IsSubclassable<T> for Widget {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();

        klass.compute_expand = Some(widget_compute_expand::<T>);
        klass.direction_changed = Some(widget_direction_changed::<T>);
    }
}

unsafe extern "C" fn widget_compute_expand<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    hexpand_ptr: *mut glib_sys::gboolean,
    vexpand_ptr: *mut glib_sys::gboolean,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let mut hexpand_p: bool = from_glib(*hexpand_ptr);
    let mut vexpand_p: bool = from_glib(*vexpand_ptr);

    imp.compute_expand(wrap.unsafe_cast_ref(), &mut hexpand_p, &mut vexpand_p);
    *hexpand_ptr = hexpand_p.to_glib();
    *vexpand_ptr = vexpand_p.to_glib();
}

unsafe extern "C" fn widget_direction_changed<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    directnptr: gtk_sys::GtkTextDirection,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let dirwrap = from_glib(directnptr);

    imp.direction_changed(wrap.unsafe_cast_ref(), dirwrap)
}
