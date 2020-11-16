use gtk_sys;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::widget::WidgetImpl;
use Button;
use Widget;

pub trait ButtonImpl: ButtonImplExt + WidgetImpl {
    fn activate(&self, button: &Self::Type) {
        self.parent_activate(button)
    }

    fn clicked(&self, button: &Self::Type) {
        self.parent_clicked(button)
    }
}

pub trait ButtonImplExt: ObjectSubclass {
    fn parent_activate(&self, button: &Self::Type);
    fn parent_clicked(&self, button: &Self::Type);
}

impl<T: ButtonImpl> ButtonImplExt for T {
    fn parent_activate(&self, button: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkButtonClass;
            if let Some(f) = (*parent_class).activate {
                f(button.unsafe_cast_ref::<Button>().to_glib_none().0)
            }
        }
    }

    fn parent_clicked(&self, button: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkButtonClass;
            if let Some(f) = (*parent_class).clicked {
                f(button.unsafe_cast_ref::<Button>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: ButtonImpl> IsSubclassable<T> for Button {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.activate = Some(button_activate::<T>);
        klass.clicked = Some(button_clicked::<T>);
    }
}

unsafe extern "C" fn button_activate<T: ButtonImpl>(ptr: *mut gtk_sys::GtkButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Button> = from_glib_borrow(ptr);

    imp.activate(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn button_clicked<T: ButtonImpl>(ptr: *mut gtk_sys::GtkButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Button> = from_glib_borrow(ptr);

    imp.clicked(wrap.unsafe_cast_ref())
}
