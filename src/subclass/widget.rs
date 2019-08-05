use gtk_sys;

use glib::translate::*;

use glib::subclass::prelude::*;

use crate::Inhibit;
use crate::Orientation;
use Widget;
use WidgetClass;

pub trait WidgetImpl:
    WidgetImplExt + ObjectImpl + 'static
{
    fn adjust_baseline_allocation(&self, widget: &Widget, baseline: &mut i32) {
        self.parent_adjust_baseline_allocation(widget, baseline)
    }

    fn adjust_baseline_request(&self, widget: &Widget, minimum_baseline: &mut i32, natural_baseline: &mut i32) {
        self.parent_adjust_baseline_request(widget, minimum_baseline, natural_baseline)
    }

    fn adjust_size_allocation(&self, widget: &Widget, orientation: Orientation, minimum_size: &mut i32, natural_size: &mut i32, allocated_pos: &mut i32, allocated_size: &mut i32) {
        self.parent_adjust_size_allocation(widget, orientation, minimum_size, natural_size, allocated_pos, allocated_size)
    }

    fn adjust_size_request(&self, widget: &Widget, orientation: Orientation, minimum_size: &mut i32, natural_size: &mut i32) {
        self.parent_adjust_size_request(widget, orientation, minimum_size, natural_size)
    }

    fn button_press_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit {
        self.parent_button_press_event(widget, event)
    }

    fn button_release_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit {
        self.parent_button_release_event(widget, event)
    }

    fn can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool {
        self.parent_can_activate_accel(widget, signal_id)
    }
}

pub trait WidgetImplExt {
    fn parent_adjust_baseline_allocation(&self, widget: &Widget, baseline: &mut i32);
    fn parent_adjust_baseline_request(&self, widget: &Widget, minimum_baseline: &mut i32, natural_baseline: &mut i32);
    fn parent_adjust_size_allocation(&self, widget: &Widget, orientation: Orientation, minimum_size: &mut i32, natural_size: &mut i32, allocated_pos: &mut i32, allocated_size: &mut i32);
    fn parent_adjust_size_request(&self, widget: &Widget, orientation: Orientation, minimum_size: &mut i32, natural_size: &mut i32);
    fn parent_button_press_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit;
    fn parent_button_release_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit;
    fn parent_can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool;
}

impl <T: WidgetImpl + ObjectImpl> WidgetImplExt for T {
    fn parent_adjust_baseline_allocation(&self, widget: &Widget, baseline: &mut i32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_baseline_allocation
                .expect("No parent class impl for \"adjust_baseline_allocation\"");
            f(widget.to_glib_none().0, baseline)
        }
    }

    fn parent_adjust_baseline_request(&self, widget: &Widget, minimum_baseline: &mut i32, natural_baseline: &mut i32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_baseline_request
                .expect("No parent class impl for \"adjust_baseline_request\"");
            f(
                widget.to_glib_none().0,
                minimum_baseline,
                natural_baseline,
            )
        }
    }

    fn parent_adjust_size_allocation(&self, widget: &Widget, orientation: Orientation, minimum_size: &mut i32, natural_size: &mut i32, allocated_pos: &mut i32, allocated_size: &mut i32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_size_allocation
                .expect("No parent class impl for \"adjust_size_allocation\"");
            f(
                widget.to_glib_none().0,
                orientation.to_glib(),
                minimum_size,
                natural_size,
                allocated_pos,
                allocated_size,
            )
        }
    }

    fn parent_adjust_size_request(&self, widget: &Widget, orientation: Orientation, minimum_size: &mut i32, natural_size: &mut i32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_size_request
                .expect("No parent class impl for \"adjust_size_request\"");
            f(
                widget.to_glib_none().0,
                orientation.to_glib(),
                minimum_size as *mut i32,
                natural_size as *mut i32,
            )
        }
    }

    fn parent_button_press_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .button_press_event
                .expect("No parent class impl for \"button_press_event\"");
            let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
            Inhibit(f(widget.to_glib_none().0, ev_glib) != 0)
        }
    }

    fn parent_button_release_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .button_release_event
                .expect("No parent class impl for \"button_release_event\"");
            let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
            Inhibit(f(widget.to_glib_none().0, ev_glib) != 0)
        }
    }

    fn parent_can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .can_activate_accel
                .expect("No parent class impl for \"can_activate_accel\"");
            f(widget.to_glib_none().0, signal_id) != 0
        }
    }
}

unsafe impl<T: ObjectSubclass + WidgetImpl> IsSubclassable<T> for WidgetClass {
    fn override_vfuncs(&mut self) {
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkWidgetClass);
            klass.adjust_baseline_allocation = Some(widget_adjust_baseline_allocation::<T>);
            klass.adjust_baseline_request = Some(widget_adjust_baseline_request::<T>);
            klass.adjust_size_allocation = Some(widget_adjust_size_allocation::<T>);
            klass.adjust_size_request = Some(widget_adjust_size_request::<T>);
            klass.button_press_event = Some(widget_button_press_event::<T>);
            klass.button_release_event = Some(widget_button_release_event::<T>);
            klass.can_activate_accel = Some(widget_can_activate_accel::<T>);
        }
    }
}

unsafe extern "C" fn widget_adjust_baseline_allocation<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    baseptr: *mut i32,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.adjust_baseline_allocation(&wrap, &mut *baseptr)
}

unsafe extern "C" fn widget_adjust_baseline_request<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    minptr: *mut i32,
    natptr: *mut i32,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.adjust_baseline_request(&wrap, &mut *minptr, &mut *natptr)
}

unsafe extern "C" fn widget_adjust_size_allocation<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    orientation: gtk_sys::GtkOrientation,
    minptr: *mut i32,
    natptr: *mut i32,
    posptr: *mut i32,
    sizeptr: *mut i32,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let wrap_orientation: Orientation = from_glib(orientation);

    imp.adjust_size_allocation(&wrap, wrap_orientation, &mut *minptr, &mut *natptr, &mut *posptr, &mut *sizeptr)
}

unsafe extern "C" fn widget_adjust_size_request<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    orientation: gtk_sys::GtkOrientation,
    minptr: *mut i32,
    natptr: *mut i32,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let wrap_orientation: Orientation = from_glib(orientation);

    imp.adjust_size_request(&wrap, wrap_orientation, &mut *minptr, &mut *natptr)
}

unsafe extern "C" fn widget_button_press_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    btnptr: *mut gdk_sys::GdkEventButton,
) -> glib_sys::gboolean
    where T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let evwrap: gdk::EventButton = from_glib_borrow(btnptr);

    imp.button_press_event(&wrap, &evwrap).0 as glib_sys::gboolean
}

unsafe extern "C" fn widget_button_release_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    btnptr: *mut gdk_sys::GdkEventButton,
) -> glib_sys::gboolean
    where T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let evwrap: gdk::EventButton = from_glib_borrow(btnptr);

    imp.button_release_event(&wrap, &evwrap).0 as glib_sys::gboolean
}

unsafe extern "C" fn widget_can_activate_accel<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    signal_id: u32,
) -> glib_sys::gboolean
    where T: WidgetImpl
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.can_activate_accel(&wrap, signal_id) as glib_sys::gboolean
}
