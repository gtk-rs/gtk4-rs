use crate::ffi;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::widget::WidgetImpl;
use crate::{DrawingArea, Widget};

pub trait DrawingAreaImpl: DrawingAreaImplExt + WidgetImpl {
    fn resize(&self, drawing_area: &Self::Type, width: i32, height: i32) {
        self.parent_resize(drawing_area, width, height)
    }
}

pub trait DrawingAreaImplExt: ObjectSubclass {
    fn parent_resize(&self, drawing_area: &Self::Type, width: i32, height: i32);
}

impl<T: DrawingAreaImpl> DrawingAreaImplExt for T {
    fn parent_resize(&self, drawing_area: &Self::Type, width: i32, height: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkDrawingAreaClass;
            if let Some(f) = (*parent_class).resize {
                f(
                    drawing_area
                        .unsafe_cast_ref::<DrawingArea>()
                        .to_glib_none()
                        .0,
                    width,
                    height,
                )
            }
        }
    }
}

unsafe impl<T: DrawingAreaImpl> IsSubclassable<T> for DrawingArea {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.resize = Some(drawing_area_resize::<T>);
    }
}

unsafe extern "C" fn drawing_area_resize<T: DrawingAreaImpl>(
    ptr: *mut ffi::GtkDrawingArea,
    width: i32,
    height: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<DrawingArea> = from_glib_borrow(ptr);

    imp.resize(wrap.unsafe_cast_ref(), width, height)
}
