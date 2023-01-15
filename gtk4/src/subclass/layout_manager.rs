// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`LayoutManager`](crate::LayoutManager).

use crate::{
    prelude::*, subclass::prelude::*, LayoutChild, LayoutManager, Orientation, SizeRequestMode,
    Widget,
};
use glib::translate::*;
use libc::c_int;

pub trait LayoutManagerImpl: LayoutManagerImplExt + ObjectImpl {
    fn allocate(&self, widget: &Widget, width: i32, height: i32, baseline: i32) {
        self.parent_allocate(widget, width, height, baseline)
    }

    fn create_layout_child(&self, widget: &Widget, for_child: &Widget) -> LayoutChild {
        self.parent_create_layout_child(widget, for_child)
    }
    // rustdoc-stripper-ignore-next
    /// Only set if the child implemented LayoutChildImpl
    fn layout_child_type() -> Option<glib::Type> {
        None
    }

    #[doc(alias = "get_request_mode")]
    fn request_mode(&self, widget: &Widget) -> SizeRequestMode {
        self.parent_request_mode(widget)
    }

    fn measure(
        &self,
        widget: &Widget,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        self.parent_measure(widget, orientation, for_size)
    }

    fn root(&self) {
        self.parent_root()
    }

    fn unroot(&self) {
        self.parent_unroot()
    }
}

pub trait LayoutManagerImplExt: ObjectSubclass {
    fn parent_allocate(&self, widget: &Widget, width: i32, height: i32, baseline: i32);

    fn parent_create_layout_child(&self, widget: &Widget, for_child: &Widget) -> LayoutChild;

    fn parent_request_mode(&self, widget: &Widget) -> SizeRequestMode;

    fn parent_measure(
        &self,
        widget: &Widget,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32);

    fn parent_root(&self);

    fn parent_unroot(&self);
}

impl<T: LayoutManagerImpl> LayoutManagerImplExt for T {
    fn parent_allocate(&self, widget: &Widget, width: i32, height: i32, baseline: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkLayoutManagerClass;
            if let Some(f) = (*parent_class).allocate {
                f(
                    self.obj()
                        .unsafe_cast_ref::<LayoutManager>()
                        .to_glib_none()
                        .0,
                    widget.to_glib_none().0,
                    width,
                    height,
                    baseline,
                )
            }
        }
    }

    fn parent_create_layout_child(&self, widget: &Widget, for_child: &Widget) -> LayoutChild {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkLayoutManagerClass;
            let f = (*parent_class)
                .create_layout_child
                .expect("No parent class impl for \"create_layout_child\"");
            from_glib_none(f(
                self.obj()
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0,
                widget.to_glib_none().0,
                for_child.to_glib_none().0,
            ))
        }
    }

    fn parent_request_mode(&self, widget: &Widget) -> SizeRequestMode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkLayoutManagerClass;
            let f = (*parent_class)
                .get_request_mode
                .expect("No parent class impl for \"get_request_mode\"");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0,
                widget.to_glib_none().0,
            ))
        }
    }

    fn parent_measure(
        &self,
        widget: &Widget,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkLayoutManagerClass;
            let f = (*parent_class)
                .measure
                .expect("No parent class impl for \"measure\"");

            let mut minimum = 0;
            let mut natural = 0;
            let mut minimum_baseline = -1;
            let mut natural_baseline = -1;
            f(
                self.obj()
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0,
                widget.to_glib_none().0,
                orientation.into_glib(),
                for_size,
                &mut minimum,
                &mut natural,
                &mut minimum_baseline,
                &mut natural_baseline,
            );
            (minimum, natural, minimum_baseline, natural_baseline)
        }
    }

    fn parent_root(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkLayoutManagerClass;
            if let Some(f) = (*parent_class).root {
                f(self
                    .obj()
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0)
            }
        }
    }

    fn parent_unroot(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkLayoutManagerClass;
            if let Some(f) = (*parent_class).unroot {
                f(self
                    .obj()
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: LayoutManagerImpl> IsSubclassable<T> for LayoutManager {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();

        let klass = class.as_mut();
        klass.allocate = Some(layout_manager_allocate::<T>);
        klass.create_layout_child = Some(layout_manager_create_layout_child::<T>);
        if let Some(type_) = T::layout_child_type() {
            klass.layout_child_type = type_.into_glib();
        }
        klass.get_request_mode = Some(layout_manager_get_request_mode::<T>);
        klass.measure = Some(layout_manager_measure::<T>);
        klass.root = Some(layout_manager_root::<T>);
        klass.unroot = Some(layout_manager_unroot::<T>);
    }
}

unsafe extern "C" fn layout_manager_allocate<T: LayoutManagerImpl>(
    ptr: *mut ffi::GtkLayoutManager,
    widgetptr: *mut ffi::GtkWidget,
    width: i32,
    height: i32,
    baseline: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);

    imp.allocate(&widget, width, height, baseline)
}

unsafe extern "C" fn layout_manager_create_layout_child<T: LayoutManagerImpl>(
    ptr: *mut ffi::GtkLayoutManager,
    widgetptr: *mut ffi::GtkWidget,
    for_childptr: *mut ffi::GtkWidget,
) -> *mut ffi::GtkLayoutChild {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);
    let for_child: Borrowed<Widget> = from_glib_borrow(for_childptr);

    imp.create_layout_child(&widget, &for_child).into_glib_ptr()
}

unsafe extern "C" fn layout_manager_get_request_mode<T: LayoutManagerImpl>(
    ptr: *mut ffi::GtkLayoutManager,
    widgetptr: *mut ffi::GtkWidget,
) -> ffi::GtkSizeRequestMode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);

    imp.request_mode(&widget).into_glib()
}

unsafe extern "C" fn layout_manager_measure<T: LayoutManagerImpl>(
    ptr: *mut ffi::GtkLayoutManager,
    widgetptr: *mut ffi::GtkWidget,
    orientation: ffi::GtkOrientation,
    for_size: i32,
    minimum_ptr: *mut c_int,
    natural_ptr: *mut c_int,
    minimum_baseline_ptr: *mut c_int,
    natural_baseline_ptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);

    let (minimum, natural, minimum_baseline, natural_baseline) =
        imp.measure(&widget, from_glib(orientation), for_size);
    if !minimum_ptr.is_null() {
        *minimum_ptr = minimum;
    }
    if !natural_ptr.is_null() {
        *natural_ptr = natural;
    }
    if !minimum_baseline_ptr.is_null() {
        *minimum_baseline_ptr = minimum_baseline;
    }
    if !natural_baseline_ptr.is_null() {
        *natural_baseline_ptr = natural_baseline;
    }
}

unsafe extern "C" fn layout_manager_root<T: LayoutManagerImpl>(ptr: *mut ffi::GtkLayoutManager) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.root()
}

unsafe extern "C" fn layout_manager_unroot<T: LayoutManagerImpl>(ptr: *mut ffi::GtkLayoutManager) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.unroot()
}
