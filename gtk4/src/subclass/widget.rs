use gtk_sys;

use glib::translate::*;

use glib::subclass::prelude::*;
use glib::ObjectClass;

use crate::TextDirection;
use Widget;
use WidgetClass;

pub trait WidgetImpl: WidgetImplExt + ObjectImpl {
    fn compute_expand(&self, widget: &Widget, hexpand_p: &mut bool, vexpand_p: &mut bool) {
        self.parent_compute_expand(widget, hexpand_p, vexpand_p)
    }

    fn direction_changed(&self, widget: &Widget, previous_direction: TextDirection) {
        self.parent_direction_changed(widget, previous_direction)
    }

    // fn can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool {
    //     self.parent_can_activate_accel(widget, signal_id)
    // }
}

pub trait WidgetImplExt {
    // fn parent_can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool;
    fn parent_compute_expand(&self, widget: &Widget, hexpand_p: &mut bool, vexpand_p: &mut bool);
    fn parent_direction_changed(&self, widget: &Widget, previous_direction: TextDirection);
}

impl<T: WidgetImpl + ObjectImpl> WidgetImplExt for T {
    // fn parent_can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool {
    //     unsafe {
    //         let data = self.get_type_data();
    //         let parent_class =
    //             data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
    //         let f = (*parent_class)
    //             .can_activate_accel
    //             .expect("No parent class impl for \"can_activate_accel\"");
    //         f(widget.to_glib_none().0, signal_id) != 0
    //     }
    // }

    fn parent_compute_expand(&self, widget: &Widget, hexpand_p: &mut bool, vexpand_p: &mut bool) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .compute_expand
                .expect("No parent class impl for \"compute_expand\"");
            let mut h: i32 = hexpand_p.to_glib();
            let mut v: i32 = vexpand_p.to_glib();
            f(widget.to_glib_none().0, &mut h, &mut v);
            *hexpand_p = from_glib(h);
            *vexpand_p = from_glib(v);
        }
    }

    fn parent_direction_changed(&self, widget: &Widget, previous_direction: TextDirection) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .direction_changed
                .expect("No parent class impl for \"direction_changed\"");
            f(widget.to_glib_none().0, previous_direction.to_glib())
        }
    }
}

unsafe impl<T: ObjectSubclass + WidgetImpl> IsSubclassable<T> for WidgetClass {
    fn override_vfuncs(&mut self) {
        <ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkWidgetClass);
            // klass.can_activate_accel = Some(widget_can_activate_accel::<T>);
            klass.compute_expand = Some(widget_compute_expand::<T>);
            klass.direction_changed = Some(widget_direction_changed::<T>);
        }
    }
}

// unsafe extern "C" fn widget_can_activate_accel<T: ObjectSubclass>(
//     ptr: *mut gtk_sys::GtkWidget,
//     signal_id: u32,
// ) -> glib_sys::gboolean
//     where T: WidgetImpl
// {
//     let instance = &*(ptr as *mut T::Instance);
//     let imp = instance.get_impl();
//     let wrap: Widget = from_glib_borrow(ptr);

//     imp.can_activate_accel(&wrap, signal_id) as glib_sys::gboolean
// }

unsafe extern "C" fn widget_compute_expand<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    hexpand_ptr: *mut glib_sys::gboolean,
    vexpand_ptr: *mut glib_sys::gboolean,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);
    let mut hexpand_p: bool = from_glib(*hexpand_ptr);
    let mut vexpand_p: bool = from_glib(*vexpand_ptr);

    imp.compute_expand(&wrap, &mut hexpand_p, &mut vexpand_p);
    *hexpand_ptr = hexpand_p.to_glib();
    *vexpand_ptr = vexpand_p.to_glib();
}

unsafe extern "C" fn widget_direction_changed<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    directnptr: gtk_sys::GtkTextDirection,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow(ptr);
    let dirwrap = from_glib(directnptr);

    imp.direction_changed(&wrap, dirwrap)
}
