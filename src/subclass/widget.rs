use gtk_sys;

use glib::translate::*;

use glib::subclass::prelude::*;
use glib::ObjectClass;

use crate::DragResult;
use crate::SelectionData;
use crate::TextDirection;
use glib::signal::Inhibit;
use Widget;
use WidgetClass;

pub trait WidgetImpl: WidgetImplExt + ObjectImpl + 'static {
    fn compute_expand(&self, widget: &Widget, hexpand_p: &mut bool, vexpand_p: &mut bool) {
        self.parent_compute_expand(widget, hexpand_p, vexpand_p)
    }

    fn destroy(&self, widget: &Widget) {
        self.parent_destroy(widget)
    }

    fn direction_changed(&self, widget: &Widget, previous_direction: TextDirection) {
        self.parent_direction_changed(widget, previous_direction)
    }

    fn drag_begin(&self, widget: &Widget, context: &gdk::Drag) {
        self.parent_drag_begin(widget, context)
    }

    fn drag_data_delete(&self, widget: &Widget, context: &gdk::Drag) {
        self.parent_drag_data_delete(widget, context)
    }

    fn drag_data_get(&self, widget: &Widget, context: &gdk::Drag, selection_data: &SelectionData) {
        self.parent_drag_data_get(widget, context, selection_data)
    }

    fn drag_data_received(
        &self,
        widget: &Widget,
        drop: &gdk::Drop,
        selection_data: &SelectionData,
    ) {
        self.parent_drag_data_received(widget, drop, selection_data)
    }

    fn drag_drop(&self, widget: &Widget, drop: &gdk::Drop, x: i32, y: i32) -> Inhibit {
        self.parent_drag_drop(widget, drop, x, y)
    }

    fn drag_end(&self, widget: &Widget, context: &gdk::Drag) {
        self.parent_drag_end(widget, context)
    }

    fn drag_failed(&self, widget: &Widget, context: &gdk::Drag, result: DragResult) -> Inhibit {
        self.parent_drag_failed(widget, context, result)
    }

    fn drag_leave(&self, widget: &Widget, drop: &gdk::Drop) {
        self.parent_drag_leave(widget, drop)
    }

    fn drag_motion(&self, widget: &Widget, drop: &gdk::Drop, x: i32, y: i32) -> Inhibit {
        self.parent_drag_motion(widget, drop, x, y)
    }

    // fn can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool {
    //     self.parent_can_activate_accel(widget, signal_id)
    // }
}

pub trait WidgetImplExt {
    // fn parent_can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool;
    fn parent_compute_expand(&self, widget: &Widget, hexpand_p: &mut bool, vexpand_p: &mut bool);
    fn parent_destroy(&self, widget: &Widget);
    fn parent_direction_changed(&self, widget: &Widget, previous_direction: TextDirection);
    fn parent_drag_begin(&self, widget: &Widget, context: &gdk::Drag);
    fn parent_drag_data_delete(&self, widget: &Widget, context: &gdk::Drag);
    fn parent_drag_data_get(
        &self,
        widget: &Widget,
        context: &gdk::Drag,
        selection_data: &SelectionData,
    );
    fn parent_drag_data_received(
        &self,
        widget: &Widget,
        drop: &gdk::Drop,
        selection_data: &SelectionData,
    );
    fn parent_drag_drop(&self, widget: &Widget, drop: &gdk::Drop, x: i32, y: i32) -> Inhibit;
    fn parent_drag_end(&self, widget: &Widget, context: &gdk::Drag);
    fn parent_drag_failed(
        &self,
        widget: &Widget,
        context: &gdk::Drag,
        result: DragResult,
    ) -> Inhibit;
    fn parent_drag_leave(&self, widget: &Widget, drop: &gdk::Drop);
    fn parent_drag_motion(&self, widget: &Widget, drop: &gdk::Drop, x: i32, y: i32) -> Inhibit;
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
            let data = self.get_type_data();
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

    fn parent_drag_begin(&self, widget: &Widget, context: &gdk::Drag) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_begin
                .expect("No parent class impl for \"drag_begin\"");
            f(widget.to_glib_none().0, context.to_glib_none().0)
        }
    }

    fn parent_drag_data_delete(&self, widget: &Widget, context: &gdk::Drag) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_data_delete
                .expect("No parent class impl for \"drag_data_delete\"");
            f(widget.to_glib_none().0, context.to_glib_none().0)
        }
    }

    fn parent_drag_data_get(
        &self,
        widget: &Widget,
        context: &gdk::Drag,
        selection_data: &SelectionData,
    ) {
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
            )
        }
    }

    fn parent_drag_data_received(
        &self,
        widget: &Widget,
        drop: &gdk::Drop,
        selection_data: &SelectionData,
    ) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_data_received
                .expect("No parent class impl for \"drag_data_received\"");
            let selection_mut = glib::translate::mut_override(selection_data.to_glib_none().0);
            f(
                widget.to_glib_none().0,
                drop.to_glib_none().0,
                selection_mut,
            )
        }
    }

    fn parent_drag_drop(&self, widget: &Widget, drop: &gdk::Drop, x: i32, y: i32) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_drop
                .expect("No parent class impl for \"drag_drop\"");
            Inhibit(from_glib(f(
                widget.to_glib_none().0,
                drop.to_glib_none().0,
                x,
                y,
            )))
        }
    }

    fn parent_drag_end(&self, widget: &Widget, context: &gdk::Drag) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_end
                .expect("No parent class impl for \"drag_end\"");
            f(widget.to_glib_none().0, context.to_glib_none().0)
        }
    }

    fn parent_drag_failed(
        &self,
        widget: &Widget,
        context: &gdk::Drag,
        result: DragResult,
    ) -> Inhibit {
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

    fn parent_drag_leave(&self, widget: &Widget, drop: &gdk::Drop) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_leave
                .expect("No parent class impl for \"drag_leave\"");
            f(widget.to_glib_none().0, drop.to_glib_none().0)
        }
    }

    fn parent_drag_motion(&self, widget: &Widget, drop: &gdk::Drop, x: i32, y: i32) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .drag_motion
                .expect("No parent class impl for \"drag_motion\"");
            Inhibit(from_glib(f(
                widget.to_glib_none().0,
                drop.to_glib_none().0,
                x,
                y,
            )))
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
            klass.destroy = Some(widget_destroy::<T>);
            klass.direction_changed = Some(widget_direction_changed::<T>);
            klass.drag_begin = Some(widget_drag_begin::<T>);
            klass.drag_data_delete = Some(widget_drag_data_delete::<T>);
            klass.drag_data_get = Some(widget_drag_data_get::<T>);
            klass.drag_data_received = Some(widget_drag_data_received::<T>);
            klass.drag_drop = Some(widget_drag_drop::<T>);
            klass.drag_end = Some(widget_drag_end::<T>);
            klass.drag_failed = Some(widget_drag_failed::<T>);
            klass.drag_leave = Some(widget_drag_leave::<T>);
            klass.drag_motion = Some(widget_drag_motion::<T>);
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
    let wrap: Widget = from_glib_borrow(ptr);
    let mut hexpand_p: bool = from_glib(*hexpand_ptr);
    let mut vexpand_p: bool = from_glib(*vexpand_ptr);

    imp.compute_expand(&wrap, &mut hexpand_p, &mut vexpand_p);
    *hexpand_ptr = hexpand_p.to_glib();
    *vexpand_ptr = vexpand_p.to_glib();
}

unsafe extern "C" fn widget_destroy<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWidget)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.destroy(&wrap)
}

unsafe extern "C" fn widget_direction_changed<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    directnptr: gtk_sys::GtkTextDirection,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let dirwrap: TextDirection = from_glib(directnptr);

    imp.direction_changed(&wrap, dirwrap)
}

unsafe extern "C" fn widget_drag_begin<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDrag,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::Drag = from_glib_borrow(ctxptr);

    imp.drag_begin(&wrap, &context)
}

unsafe extern "C" fn widget_drag_data_delete<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDrag,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::Drag = from_glib_borrow(ctxptr);

    imp.drag_data_delete(&wrap, &context)
}

unsafe extern "C" fn widget_drag_data_get<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDrag,
    selectptr: *mut gtk_sys::GtkSelectionData,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::Drag = from_glib_borrow(ctxptr);
    let selection_data: SelectionData = from_glib_borrow(selectptr);

    imp.drag_data_get(&wrap, &context, &selection_data)
}

unsafe extern "C" fn widget_drag_data_received<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    dropptr: *mut gdk_sys::GdkDrop,
    selectptr: *mut gtk_sys::GtkSelectionData,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let drop: gdk::Drop = from_glib_borrow(dropptr);
    let selection_data: SelectionData = from_glib_borrow(selectptr);

    imp.drag_data_received(&wrap, &drop, &selection_data)
}

unsafe extern "C" fn widget_drag_drop<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    dropptr: *mut gdk_sys::GdkDrop,
    x: i32,
    y: i32,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let drop: gdk::Drop = from_glib_borrow(dropptr);

    imp.drag_drop(&wrap, &drop, x, y).to_glib()
}

unsafe extern "C" fn widget_drag_end<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDrag,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::Drag = from_glib_borrow(ctxptr);

    imp.drag_end(&wrap, &context)
}

unsafe extern "C" fn widget_drag_failed<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDrag,
    resultptr: gtk_sys::GtkDragResult,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::Drag = from_glib_borrow(ctxptr);
    let result: DragResult = from_glib(resultptr);

    imp.drag_failed(&wrap, &context, result).to_glib()
}

unsafe extern "C" fn widget_drag_leave<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    dropptr: *mut gdk_sys::GdkDrop,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let drop: gdk::Drop = from_glib_borrow(dropptr);

    imp.drag_leave(&wrap, &drop)
}

unsafe extern "C" fn widget_drag_motion<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    dropptr: *mut gdk_sys::GdkDrop,
    x: i32,
    y: i32,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let drop: gdk::Drop = from_glib_borrow(dropptr);

    imp.drag_motion(&wrap, &drop, x, y).to_glib()
}
