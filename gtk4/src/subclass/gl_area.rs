// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`GLArea`](crate::GLArea).

use crate::{prelude::*, subclass::prelude::*, GLArea};
use gdk::GLContext;
use glib::translate::*;

#[allow(clippy::upper_case_acronyms)]
pub trait GLAreaImpl: GLAreaImplExt + WidgetImpl {
    fn create_context(&self) -> Option<GLContext> {
        self.parent_create_context()
    }

    fn render(&self, context: &GLContext) -> bool {
        self.parent_render(context)
    }

    fn resize(&self, width: i32, height: i32) {
        self.parent_resize(width, height)
    }
}

#[allow(clippy::upper_case_acronyms)]
pub trait GLAreaImplExt: ObjectSubclass {
    fn parent_create_context(&self) -> Option<GLContext>;
    fn parent_render(&self, context: &GLContext) -> bool;
    fn parent_resize(&self, width: i32, height: i32);
}

impl<T: GLAreaImpl> GLAreaImplExt for T {
    fn parent_create_context(&self) -> Option<GLContext> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkGLAreaClass;
            if let Some(f) = (*parent_class).create_context {
                return Some(from_glib_none(f(self
                    .obj()
                    .unsafe_cast_ref::<GLArea>()
                    .to_glib_none()
                    .0)));
            };
            None
        }
    }

    fn parent_render(&self, context: &GLContext) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkGLAreaClass;
            let f = (*parent_class)
                .render
                .expect("No parent class impl for \"render\"");
            from_glib(f(
                self.obj().unsafe_cast_ref::<GLArea>().to_glib_none().0,
                context.to_glib_none().0,
            ))
        }
    }

    fn parent_resize(&self, width: i32, height: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkGLAreaClass;
            if let Some(f) = (*parent_class).resize {
                f(
                    self.obj().unsafe_cast_ref::<GLArea>().to_glib_none().0,
                    width,
                    height,
                )
            }
        }
    }
}

unsafe impl<T: GLAreaImpl> IsSubclassable<T> for GLArea {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.create_context = Some(gl_area_create_context::<T>);
        klass.render = Some(gl_area_render::<T>);
        klass.resize = Some(gl_area_resize::<T>);
    }
}

unsafe extern "C" fn gl_area_create_context<T: GLAreaImpl>(
    ptr: *mut ffi::GtkGLArea,
) -> *mut gdk::ffi::GdkGLContext {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.create_context().into_glib_ptr()
}

unsafe extern "C" fn gl_area_render<T: GLAreaImpl>(
    ptr: *mut ffi::GtkGLArea,
    context: *mut gdk::ffi::GdkGLContext,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.render(&from_glib_borrow(context)).into_glib()
}

unsafe extern "C" fn gl_area_resize<T: GLAreaImpl>(
    ptr: *mut ffi::GtkGLArea,
    width: i32,
    height: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.resize(width, height)
}
