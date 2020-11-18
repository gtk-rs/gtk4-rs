use gtk_sys;

use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Object};

use StyleContext;

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
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkStyleContextClass;
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
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.changed = Some(style_context_changed::<T>);
    }
}

unsafe extern "C" fn style_context_changed<T: StyleContextImpl>(
    ptr: *mut gtk_sys::GtkStyleContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<StyleContext> = from_glib_borrow(ptr);

    imp.changed(wrap.unsafe_cast_ref())
}
