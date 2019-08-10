use gtk_sys;

use glib::translate::*;

use glib::subclass::prelude::*;

use crate::DragResult;
use crate::Inhibit;
use crate::Orientation;
use crate::SelectionData;
use crate::TextDirection;
use cairo;
use cairo_sys;
use Widget;
use WidgetClass;

pub trait WidgetImpl: WidgetImplExt + ObjectImpl + 'static {
    fn adjust_baseline_allocation(&self, widget: &Widget, baseline: &mut i32) {
        self.parent_adjust_baseline_allocation(widget, baseline)
    }

    fn adjust_baseline_request(
        &self,
        widget: &Widget,
        minimum_baseline: &mut i32,
        natural_baseline: &mut i32,
    ) {
        self.parent_adjust_baseline_request(widget, minimum_baseline, natural_baseline)
    }

    fn adjust_size_allocation(
        &self,
        widget: &Widget,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
        allocated_pos: &mut i32,
        allocated_size: &mut i32,
    ) {
        self.parent_adjust_size_allocation(
            widget,
            orientation,
            minimum_size,
            natural_size,
            allocated_pos,
            allocated_size,
        )
    }

    fn adjust_size_request(
        &self,
        widget: &Widget,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
    ) {
        self.parent_adjust_size_request(widget, orientation, minimum_size, natural_size)
    }

    fn button_press_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit {
        self.parent_button_press_event(widget, event)
    }

    fn button_release_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit {
        self.parent_button_release_event(widget, event)
    }

    fn child_notify(&self, widget: &Widget, child_property: &glib::ParamSpec) {
        self.parent_child_notify(widget, child_property)
    }

    fn composited_changed(&self, widget: &Widget) {
        self.parent_composited_changed(widget)
    }

    fn compute_expand(&self, widget: &Widget, hexpand_p: &mut bool, vexpand_p: &mut bool) {
        self.parent_compute_expand(widget, hexpand_p, vexpand_p)
    }

    fn configure_event(&self, widget: &Widget, event: &gdk::EventConfigure) -> Inhibit {
        self.parent_configure_event(widget, event)
    }

    fn damage_event(&self, widget: &Widget, event: &gdk::EventExpose) -> Inhibit {
        self.parent_damage_event(widget, event)
    }

    // fn delete_event(&self, widget: &Widget, event:/*missing*/ &gdk::EventAny) -> Inhibit {
    //     self.parent_delete_event(widget, event)
    // }

    fn destroy(&self, widget: &Widget) {
        self.parent_destroy(widget)
    }

    // fn destroy_event&self, widget: &Widget, event:/*missing*/ &gdk::EventAny) -> Inhibit {
    //     self.parent_destroy_event(widget, event)
    // }

    fn direction_changed(&self, widget: &Widget, previous_direction: TextDirection) {
        self.parent_direction_changed(widget, previous_direction)
    }

    // fn dispatch_child_properties_changed(&self, widget: &Widget, n_pspecs: u32, pspecs: &Vec<&glib::ParamSpec>) {
    //     self.parent_dispatch_child_properties_changed(widget, n_pspecs, pspecs)
    // }

    fn drag_begin(&self, widget: &Widget, context: &gdk::DragContext) {
        self.parent_drag_begin(widget, context)
    }

    fn drag_data_delete(&self, widget: &Widget, context: &gdk::DragContext) {
        self.parent_drag_data_delete(widget, context)
    }

    fn drag_data_get(&self, widget: &Widget, context: &gdk::DragContext, selection_data: &SelectionData, info: u32, time: u32) {
        self.parent_drag_data_get(widget, context, selection_data, info, time)
    }

    fn drag_data_received(&self, widget: &Widget, context: &gdk::DragContext, x: i32, y: i32, selection_data: &SelectionData, info: u32, time: u32) {
        self.parent_drag_data_received(widget, context, x, y, selection_data, info, time)
    }

    fn drag_drop(&self, widget: &Widget, context: &gdk::DragContext, x: i32, y: i32, time: u32) -> Inhibit {
        self.parent_drag_drop(widget, context, x, y, time)
    }

    fn drag_end(&self, widget: &Widget, context: &gdk::DragContext) {
        self.parent_drag_end(widget, context)
    }

    fn drag_failed(&self, widget: &Widget, context: &gdk::DragContext, result: DragResult) -> Inhibit {
        self.parent_drag_failed(widget, context, result)
    }

    fn drag_leave(&self, widget: &Widget, context: &gdk::DragContext, time: u32) {
        self.parent_drag_leave(widget, context, time)
    }

    fn drag_motion(&self, widget: &Widget, context: &gdk::DragContext, x: i32, y: i32, time: u32) -> Inhibit {
        self.parent_drag_motion(widget, context, x, y, time)
    }

    fn draw(&self, widget: &Widget, cr: &cairo::Context) -> Inhibit {
        self.parent_draw(widget, cr)
    }

    // fn can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool {
    //     self.parent_can_activate_accel(widget, signal_id)
    // }
}

pub trait WidgetImplExt {
    fn parent_adjust_baseline_allocation(&self, widget: &Widget, baseline: &mut i32);
    fn parent_adjust_baseline_request(
        &self,
        widget: &Widget,
        minimum_baseline: &mut i32,
        natural_baseline: &mut i32,
    );
    fn parent_adjust_size_allocation(
        &self,
        widget: &Widget,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
        allocated_pos: &mut i32,
        allocated_size: &mut i32,
    );
    fn parent_adjust_size_request(
        &self,
        widget: &Widget,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
    );
    fn parent_button_press_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit;
    fn parent_button_release_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit;
    // fn parent_can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool;
    fn parent_child_notify(&self, widget: &Widget, child_property: &glib::ParamSpec);
    fn parent_composited_changed(&self, widget: &Widget);
    fn parent_compute_expand(&self, widget: &Widget, hexpand_p: &mut bool, vexpand_p: &mut bool);
    fn parent_configure_event(&self, widget: &Widget, event: &gdk::EventConfigure) -> Inhibit;
    fn parent_damage_event(&self, widget: &Widget, event: &gdk::EventExpose) -> Inhibit;
    // fn parent_delete_event(&self, widget: &Widget, event:/*missing*/ &gdk::EventAny) -> Inhibit;
    fn parent_destroy(&self, widget: &Widget);
    // fn parent_destroy_event(&self, widget: &Widget, event:/*missing*/ &gdk::EventAny) -> Inhibit;
    fn parent_direction_changed(&self, widget: &Widget, previous_direction: TextDirection);
    // fn parent_dispatch_child_properties_changed(&self, widget: &Widget, n_pspecs: u32, pspecs: &Vec<&glib::ParamSpec>);
    fn parent_drag_begin(&self, widget: &Widget, context: &gdk::DragContext);
    fn parent_drag_data_delete(&self, widget: &Widget, context: &gdk::DragContext);
    fn parent_drag_data_get(&self, widget: &Widget, context: &gdk::DragContext, selection_data: &SelectionData, info: u32, time: u32);
    fn parent_drag_data_received(&self, widget: &Widget, context: &gdk::DragContext, x: i32, y: i32, selection_data: &SelectionData, info: u32, time: u32);
    fn parent_drag_drop(&self, widget: &Widget, context: &gdk::DragContext, x: i32, y: i32, time: u32) -> Inhibit;
    fn parent_drag_end(&self, widget: &Widget, context: &gdk::DragContext);
    fn parent_drag_failed(&self, widget: &Widget, context: &gdk::DragContext, result: DragResult) -> Inhibit;
    fn parent_drag_leave(&self, widget: &Widget, context: &gdk::DragContext, time: u32);
    fn parent_drag_motion(&self, widget: &Widget, context: &gdk::DragContext, x: i32, y: i32, time: u32) -> Inhibit;
    fn parent_draw(&self, widget: &Widget, cr: &cairo::Context) -> Inhibit;
}

impl<T: WidgetImpl + ObjectImpl> WidgetImplExt for T {
    fn parent_adjust_baseline_allocation(&self, widget: &Widget, baseline: &mut i32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_baseline_allocation
                .expect("No parent class impl for \"adjust_baseline_allocation\"");
            f(widget.to_glib_none().0, baseline)
        }
    }

    fn parent_adjust_baseline_request(
        &self,
        widget: &Widget,
        minimum_baseline: &mut i32,
        natural_baseline: &mut i32,
    ) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_baseline_request
                .expect("No parent class impl for \"adjust_baseline_request\"");
            f(widget.to_glib_none().0, minimum_baseline, natural_baseline)
        }
    }

    fn parent_adjust_size_allocation(
        &self,
        widget: &Widget,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
        allocated_pos: &mut i32,
        allocated_size: &mut i32,
    ) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
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

    fn parent_adjust_size_request(
        &self,
        widget: &Widget,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
    ) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
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
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .button_press_event
                .expect("No parent class impl for \"button_press_event\"");
            let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
            Inhibit(from_glib(f(widget.to_glib_none().0, ev_glib)))
        }
    }

    fn parent_button_release_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .button_release_event
                .expect("No parent class impl for \"button_release_event\"");
            let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
            Inhibit(from_glib(f(widget.to_glib_none().0, ev_glib)))
        }
    }

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

    fn parent_child_notify(&self, widget: &Widget, child_property: &glib::ParamSpec) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .child_notify
                .expect("No parent class impl for \"child_notify\"");
            let pspec_glib = glib::translate::mut_override(child_property.to_glib_none().0);
            f(widget.to_glib_none().0, pspec_glib)
        }
    }

    fn parent_composited_changed(&self, widget: &Widget) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .composited_changed
                .expect("No parent class impl for \"composited_changed\"");
            f(widget.to_glib_none().0)
        }
    }

    fn parent_compute_expand(&self, widget: &Widget, hexpand_p: &mut bool, vexpand_p: &mut bool) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .compute_expand
                .expect("No parent class impl for \"compute_expand\"");
            f(
                widget.to_glib_none().0,
                *hexpand_p as i32 as *mut i32,
                *vexpand_p as i32 as *mut i32,
            )
        }
    }

    fn parent_configure_event(&self, widget: &Widget, event: &gdk::EventConfigure) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .configure_event
                .expect("No parent class impl for \"configure_event\"");
            let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
            Inhibit(from_glib(f(widget.to_glib_none().0, ev_glib)))
        }
    }

    fn parent_damage_event(&self, widget: &Widget, event: &gdk::EventExpose) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .damage_event
                .expect("No parent class impl for \"damage_event\"");
            let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
            Inhibit(from_glib(f(widget.to_glib_none().0, ev_glib)))
        }
    }

    // fn parent_delete_event(&self, widget: &Widget, event:/*missing*/ &gdk::EventAny) -> Inhibit {
    //     TODO: call GtkWidgetClass.delete_event()
    // }

    fn parent_destroy(&self, widget: &Widget) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .destroy
                .expect("No parent class impl for \"destroy\"");
            f(widget.to_glib_none().0)
        }
    }

    // fn parent_destroy_event(&self, widget: &Widget, event:/*missing*/ &gdk::EventAny) -> Inhibit {
    //     TODO: call GtkWidgetClass.destroy_event()
    // }

    fn parent_direction_changed(&self, widget: &Widget, previous_direction: TextDirection) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .direction_changed
                .expect("No parent class impl for \"direction_changed\"");
            f(widget.to_glib_none().0, previous_direction.to_glib())
        }
    }

    // fn parent_dispatch_child_properties_changed(&self, widget: &Widget, n_pspecs: u32, pspecs: &Vec<&glib::ParamSpec>) {
    //     TODO: Call GtkWidgetClass.dispatch_child_properties_changed()
    // }

    fn parent_drag_begin(&self, widget: &Widget, context: &gdk::DragContext) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_begin
                .expect("No parent class impl for \"drag_begin\"");
            f(widget.to_glib_none().0, context.to_glib_none().0)
        }
    }

    fn parent_drag_data_delete(&self, widget: &Widget, context: &gdk::DragContext) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_data_delete
                .expect("No parent class impl for \"drag_data_delete\"");
            f(widget.to_glib_none().0, context.to_glib_none().0)
        }
    }

    fn parent_drag_data_get(&self, widget: &Widget, context: &gdk::DragContext, selection_data: &SelectionData, info: u32, time: u32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_data_get
                .expect("No parent class impl for \"drag_data_get\"");
            let selection_mut = glib::translate::mut_override(selection_data.to_glib_none().0);
            f(
                widget.to_glib_none().0,
                context.to_glib_none().0,
                selection_mut,
                info,
                time,
            )
        }
    }

    fn parent_drag_data_received(&self, widget: &Widget, context: &gdk::DragContext, x: i32, y: i32, selection_data: &SelectionData, info: u32, time: u32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_data_received
                .expect("No parent class impl for \"drag_data_received\"");
            let selection_mut = glib::translate::mut_override(selection_data.to_glib_none().0);
            f(
                widget.to_glib_none().0,
                context.to_glib_none().0,
                x,
                y,
                selection_mut,
                info,
                time,
            )
        }
    }

    fn parent_drag_drop(&self, widget: &Widget, context: &gdk::DragContext, x: i32, y: i32, time: u32) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_drop
                .expect("No parent class impl for \"drag_drop\"");
            Inhibit(from_glib(f(
                widget.to_glib_none().0,
                context.to_glib_none().0,
                x,
                y,
                time,
            )))
        }
    }

    fn parent_drag_end(&self, widget: &Widget, context: &gdk::DragContext) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_end
                .expect("No parent class impl for \"drag_end\"");
            f(widget.to_glib_none().0, context.to_glib_none().0)
        }
    }

    fn parent_drag_failed(&self, widget: &Widget, context: &gdk::DragContext, result: DragResult) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_failed
                .expect("No parent class impl for \"drag_failed\"");
            Inhibit(from_glib(f(
                widget.to_glib_none().0,
                context.to_glib_none().0,
                result.to_glib(),
            )))
        }
    }

    fn parent_drag_leave(&self, widget: &Widget, context: &gdk::DragContext, time: u32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_leave
                .expect("No parent class impl for \"drag_leave\"");
            f(
                widget.to_glib_none().0,
                context.to_glib_none().0,
                time,
            )
        }
    }

    fn parent_drag_motion(&self, widget: &Widget, context: &gdk::DragContext, x: i32, y: i32, time: u32) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_motion
                .expect("No parent class impl for \"drag_motion\"");
            Inhibit(from_glib(f(
                widget.to_glib_none().0,
                context.to_glib_none().0,
                x,
                y,
                time,
            )))
        }
    }

    fn parent_draw(&self, widget: &Widget, cr: &cairo::Context) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .draw
                .expect("No parent class impl for \"draw\"");
            Inhibit(from_glib(f(widget.to_glib_none().0, cr.to_glib_none().0)))
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
            // klass.can_activate_accel = Some(widget_can_activate_accel::<T>);
            klass.child_notify = Some(widget_child_notify::<T>);
            klass.composited_changed = Some(widget_composited_changed::<T>);
            klass.compute_expand = Some(widget_compute_expand::<T>);
            klass.configure_event = Some(widget_configure_event::<T>);
            klass.damage_event = Some(widget_damage_event::<T>);
            // klass.delete_event = Some(widget_delete_event::<T>);
            klass.destroy = Some(widget_destroy::<T>);
            // klass.destroy_event = Some(widget_destroy_event::<T>);
            klass.direction_changed = Some(widget_direction_changed::<T>);
            // klass.dispatch_child_properties_changed = Some(widget_dispatch_child_properties_changed::<T>);
            klass.drag_begin = Some(widget_drag_begin::<T>);
            klass.drag_data_delete = Some(widget_drag_data_delete::<T>);
            klass.drag_data_get = Some(widget_drag_data_get::<T>);
            klass.drag_data_received = Some(widget_drag_data_received::<T>);
            klass.drag_drop = Some(widget_drag_drop::<T>);
            klass.drag_end = Some(widget_drag_end::<T>);
            klass.drag_failed = Some(widget_drag_failed::<T>);
            klass.drag_leave = Some(widget_drag_leave::<T>);
            klass.drag_motion = Some(widget_drag_motion::<T>);
            klass.draw = Some(widget_draw::<T>);
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

    imp.adjust_size_allocation(
        &wrap,
        wrap_orientation,
        &mut *minptr,
        &mut *natptr,
        &mut *posptr,
        &mut *sizeptr,
    )
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
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let evwrap: gdk::EventButton = from_glib_borrow(btnptr);

    imp.button_press_event(&wrap, &evwrap).to_glib()
}

unsafe extern "C" fn widget_button_release_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    btnptr: *mut gdk_sys::GdkEventButton,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let evwrap: gdk::EventButton = from_glib_borrow(btnptr);

    imp.button_release_event(&wrap, &evwrap).to_glib()
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

unsafe extern "C" fn widget_child_notify<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    paramptr: *mut gobject_sys::GParamSpec,
)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let paramwrap: glib::ParamSpec = from_glib_borrow(paramptr);

    imp.child_notify(&wrap, &paramwrap)
}

unsafe extern "C" fn widget_composited_changed<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.composited_changed(&wrap)
}

unsafe extern "C" fn widget_compute_expand<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    hexpand_ptr: *mut glib_sys::gboolean,
    vexpand_ptr: *mut glib_sys::gboolean,
)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let mut hexpand_p: bool = *hexpand_ptr != 0;
    let mut vexpand_p: bool = *vexpand_ptr != 0;

    imp.compute_expand(&wrap, &mut hexpand_p, &mut vexpand_p)
}

unsafe extern "C" fn widget_configure_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    confptr: *mut gdk_sys::GdkEventConfigure,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let evwrap: gdk::EventConfigure = from_glib_borrow(confptr);

    imp.configure_event(&wrap, &evwrap).to_glib()
}

unsafe extern "C" fn widget_damage_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    exposeptr: *mut gdk_sys::GdkEventExpose,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let evwrap: gdk::EventExpose = from_glib_borrow(exposeptr);

    imp.damage_event(&wrap, &evwrap).to_glib()
}

// unsafe extern "C" fn widget_delete_event<T: ObjectSubclass>(
//     ptr: *mut gtk_sys::GtkWidget,
//     anyptr: *mut gdk_sys::GdkEventAny,
// ) -> glib_sys::gboolean
// where
//     T: WidgetImpl,
// {
//     let instance = &*(ptr as *mut T::Instance);
//     let imp = instance.get_impl();
//     let wrap: Widget = from_glib_borrow(ptr);
//     let evwrap: gdk::EventAny = from_glib_borrow(anyptr);

//     imp.damage_event(&wrap, &evwrap).to_glib()
// }

unsafe extern "C" fn widget_destroy<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.destroy(&wrap)
}

// unsafe extern "C" fn widget_destroy_event<T: ObjectSubclass>(
//     ptr: *mut gtk_sys::GtkWidget,
//     anyptr: *mut gdk_sys::GdkEventAny,
// ) -> glib_sys::gboolean
// where
//     T: WidgetImpl,
// {
//     let instance = &*(ptr as *mut T::Instance);
//     let imp = instance.get_impl();
//     let wrap: Widget = from_glib_borrow(ptr);
//     let evwrap: gdk::EventAny = from_glib_borrow(anyptr);

//     imp.destroy_event(&wrap, &evwrap).to_glib()
// }

unsafe extern "C" fn widget_direction_changed<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    directnptr: gtk_sys::GtkTextDirection,
)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let dirwrap: TextDirection = from_glib(directnptr);

    imp.direction_changed(&wrap, dirwrap)
}

// unsafe extern "C" fn widget_dispatch_child_properties_changed<T: ObjectSubclass>(
//     ptr: *mut gtk_sys::GtkWidget,
//     n_pspec_ptr: *mut u32,
//     pspecsptr: *mut *mut gobject_sys::GParamSpec,
// )
// where
//     T: WidgetImpl,
// {
//     let instance = &*(ptr as *mut T::Instance);
//     let imp = instance.get_impl();
//     let wrap: Widget = from_glib_borrow(ptr);
    // TODO: Figure out how to translate pspecs

//     imp.dispatch_child_properties_changed(/* untranslated params */)
// }

unsafe extern "C" fn widget_drag_begin<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);

    imp.drag_begin(&wrap, &context)
}

unsafe extern "C" fn widget_drag_data_delete<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);

    imp.drag_data_delete(&wrap, &context)
}

unsafe extern "C" fn widget_drag_data_get<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    selectptr: *mut gtk_sys::GtkSelectionData,
    info: u32,
    time: u32,
)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);
    let selection_data: SelectionData = from_glib_borrow(selectptr);

    imp.drag_data_get(&wrap, &context, &selection_data, info, time)
}

unsafe extern "C" fn widget_drag_data_received<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    x: i32,
    y: i32,
    selectptr: *mut gtk_sys::GtkSelectionData,
    info: u32,
    time: u32,
)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);
    let selection_data: SelectionData = from_glib_borrow(selectptr);

    imp.drag_data_received(&wrap, &context, x, y, &selection_data, info, time)
}

unsafe extern "C" fn widget_drag_drop<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    x: i32,
    y: i32,
    time: u32,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);

    imp.drag_drop(&wrap, &context, x, y, time).to_glib()
}

unsafe extern "C" fn widget_drag_end<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);

    imp.drag_end(&wrap, &context)
}

unsafe extern "C" fn widget_drag_failed<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    resultptr: gtk_sys::GtkDragResult,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);
    let result: DragResult = from_glib(resultptr);

    imp.drag_failed(&wrap, &context, result).to_glib()
}

unsafe extern "C" fn widget_drag_leave<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    time: u32,
)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);

    imp.drag_leave(&wrap, &context, time)
}

unsafe extern "C" fn widget_drag_motion<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    x: i32,
    y: i32,
    time: u32,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);

    imp.drag_motion(&wrap, &context, x, y, time).to_glib()
}

unsafe extern "C" fn widget_draw<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    cr_ptr: *mut cairo_sys::cairo_t,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let cr: cairo::Context = from_glib_borrow(cr_ptr);

    imp.draw(&wrap, &cr).to_glib()
}
