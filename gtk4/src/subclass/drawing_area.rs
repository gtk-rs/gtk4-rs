// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`DrawingA£rea`](crate::DrawingA£rea).

use crate::{prelude::*, subclass::prelude::*, DrawingArea};
use glib::translate::*;

pub trait DrawingAreaImpl: DrawingAreaImplExt + WidgetImpl {
    fn resize(&self, width: i32, height: i32) {
        self.parent_resize(width, height)
    }
}

pub trait DrawingAreaImplExt: ObjectSubclass {
    fn parent_resize(&self, width: i32, height: i32);
}

impl<T: DrawingAreaImpl> DrawingAreaImplExt for T {
    fn parent_resize(&self, width: i32, height: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkDrawingAreaClass;
            if let Some(f) = (*parent_class).resize {
                f(
                    self.obj().unsafe_cast_ref::<DrawingArea>().to_glib_none().0,
                    width,
                    height,
                )
            }
        }
    }
}

unsafe impl<T: DrawingAreaImpl> IsSubclassable<T> for DrawingArea {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

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
    let imp = instance.imp();

    imp.resize(width, height)
}
