// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Object};

use crate::StyleContext;

pub trait StyleContextImpl: StyleContextImplExt + ObjectImpl {
    fn changed(&self, style_context: &Self::Type) {
        self.parent_changed(style_context)
    }
}

pub trait StyleContextImplExt: ObjectSubclass {
    fn parent_changed(&self, style_context: &Self::Type);
}

impl<T: StyleContextImpl> StyleContextImplExt for T {
    fn parent_changed(&self, style_context: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkStyleContextClass;
            if let Some(f) = (*parent_class).changed {
                f(style_context
                    .unsafe_cast_ref::<StyleContext>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: StyleContextImpl> IsSubclassable<T> for StyleContext {
    fn class_init(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.changed = Some(style_context_changed::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn style_context_changed<T: StyleContextImpl>(ptr: *mut ffi::GtkStyleContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<StyleContext> = from_glib_borrow(ptr);

    imp.changed(wrap.unsafe_cast_ref())
}
