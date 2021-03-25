// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::widget::WidgetImpl;
use crate::{GLArea, Widget};
use gdk::GLContext;

#[allow(clippy::upper_case_acronyms)]
pub trait GLAreaImpl: GLAreaImplExt + WidgetImpl {
    fn create_context(&self, gl_area: &Self::Type) -> Option<GLContext> {
        self.parent_create_context(gl_area)
    }

    fn render(&self, gl_area: &Self::Type, context: &GLContext) -> bool {
        self.parent_render(gl_area, context)
    }

    fn resize(&self, gl_area: &Self::Type, width: i32, height: i32) {
        self.parent_resize(gl_area, width, height)
    }
}

#[allow(clippy::upper_case_acronyms)]
pub trait GLAreaImplExt: ObjectSubclass {
    fn parent_create_context(&self, gl_area: &Self::Type) -> Option<GLContext>;
    fn parent_render(&self, gl_area: &Self::Type, context: &GLContext) -> bool;
    fn parent_resize(&self, gl_area: &Self::Type, width: i32, height: i32);
}

impl<T: GLAreaImpl> GLAreaImplExt for T {
    fn parent_create_context(&self, gl_area: &Self::Type) -> Option<GLContext> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkGLAreaClass;
            if let Some(f) = (*parent_class).create_context {
                return Some(from_glib_none(f(gl_area
                    .unsafe_cast_ref::<GLArea>()
                    .to_glib_none()
                    .0)));
            };
            None
        }
    }

    fn parent_render(&self, gl_area: &Self::Type, context: &GLContext) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkGLAreaClass;
            let f = (*parent_class)
                .render
                .expect("No parent class impl for \"render\"");
            from_glib(f(
                gl_area.unsafe_cast_ref::<GLArea>().to_glib_none().0,
                context.to_glib_none().0,
            ))
        }
    }

    fn parent_resize(&self, gl_area: &Self::Type, width: i32, height: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkGLAreaClass;
            if let Some(f) = (*parent_class).resize {
                f(
                    gl_area.unsafe_cast_ref::<GLArea>().to_glib_none().0,
                    width,
                    height,
                )
            }
        }
    }
}

unsafe impl<T: GLAreaImpl> IsSubclassable<T> for GLArea {
    fn class_init(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.create_context = Some(gl_area_create_context::<T>);
        klass.render = Some(gl_area_render::<T>);
        klass.resize = Some(gl_area_resize::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Widget as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn gl_area_create_context<T: GLAreaImpl>(
    ptr: *mut ffi::GtkGLArea,
) -> *mut gdk::ffi::GdkGLContext {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<GLArea> = from_glib_borrow(ptr);

    imp.create_context(wrap.unsafe_cast_ref()).to_glib_full()
}

unsafe extern "C" fn gl_area_render<T: GLAreaImpl>(
    ptr: *mut ffi::GtkGLArea,
    context: *mut gdk::ffi::GdkGLContext,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<GLArea> = from_glib_borrow(ptr);

    imp.render(wrap.unsafe_cast_ref(), &from_glib_borrow(context))
        .to_glib()
}

unsafe extern "C" fn gl_area_resize<T: GLAreaImpl>(
    ptr: *mut ffi::GtkGLArea,
    width: i32,
    height: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<GLArea> = from_glib_borrow(ptr);

    imp.resize(wrap.unsafe_cast_ref(), width, height)
}
