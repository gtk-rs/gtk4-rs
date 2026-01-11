// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`ApplicationWindow`].

use crate::{ApplicationWindow, prelude::*, subclass::prelude::*};

#[cfg(feature = "v4_22")]
use crate::ffi;

#[cfg(feature = "v4_22")]
use glib::translate::*;

pub trait ApplicationWindowImpl:
    WindowImpl
    + ObjectSubclass<Type: IsA<ApplicationWindow> + IsA<gio::ActionGroup> + IsA<gio::ActionMap>>
    + 'static
{
    #[cfg(feature = "v4_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_22")))]
    fn save_state(&self, state: &glib::VariantDict) -> bool {
        self.parent_save_state(state)
    }
}

pub trait ApplicationWindowImplExt: ApplicationWindowImpl {
    #[cfg(feature = "v4_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_22")))]
    fn parent_save_state(&self, state: &glib::VariantDict) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkApplicationWindowClass;
            if let Some(f) = (*parent_class).save_state {
                from_glib(f(
                    self.obj()
                        .unsafe_cast_ref::<ApplicationWindow>()
                        .to_glib_none()
                        .0,
                    state.to_glib_none().0,
                ))
            } else {
                false
            }
        }
    }
}

impl<T: ApplicationWindowImpl> ApplicationWindowImplExt for T {}

unsafe impl<T: ApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        #[cfg(feature = "v4_22")]
        {
            let klass = class.as_mut();
            klass.save_state = Some(application_window_save_state::<T>);
        }
    }
}

#[cfg(feature = "v4_22")]
unsafe extern "C" fn application_window_save_state<T: ApplicationWindowImpl>(
    ptr: *mut ffi::GtkApplicationWindow,
    state: *mut glib::ffi::GVariantDict,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.save_state(&from_glib_borrow(state)).into_glib()
}
