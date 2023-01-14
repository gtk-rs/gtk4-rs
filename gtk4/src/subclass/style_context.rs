// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`StyleContext`](crate::StyleContext).

use crate::{prelude::*, subclass::prelude::*, StyleContext};
use glib::translate::*;

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait StyleContextImpl: StyleContextImplExt + ObjectImpl {
    fn changed(&self) {
        self.parent_changed()
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait StyleContextImplExt: ObjectSubclass {
    fn parent_changed(&self);
}

impl<T: StyleContextImpl> StyleContextImplExt for T {
    fn parent_changed(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkStyleContextClass;
            if let Some(f) = (*parent_class).changed {
                f(self
                    .obj()
                    .unsafe_cast_ref::<StyleContext>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: StyleContextImpl> IsSubclassable<T> for StyleContext {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();
        let klass = class.as_mut();
        klass.changed = Some(style_context_changed::<T>);
    }
}

unsafe extern "C" fn style_context_changed<T: StyleContextImpl>(ptr: *mut ffi::GtkStyleContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.changed()
}
