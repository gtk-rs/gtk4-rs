// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`ComboBox`](crate::ComboBox).

use crate::{prelude::*, subclass::prelude::*, ComboBox};
use glib::{translate::*, GString};

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait ComboBoxImpl: ComboBoxImplExt + WidgetImpl {
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    fn activate(&self) {
        self.parent_activate()
    }
    fn changed(&self) {
        self.parent_changed()
    }
    fn format_entry_text(&self, path: &str) -> Option<GString> {
        self.parent_format_entry_text(path)
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait ComboBoxImplExt: ObjectSubclass {
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    fn parent_activate(&self);

    fn parent_changed(&self);

    fn parent_format_entry_text(&self, path: &str) -> Option<GString>;
}

impl<T: ComboBoxImpl> ComboBoxImplExt for T {
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    fn parent_activate(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkComboBoxClass;
            if let Some(f) = (*parent_class).activate {
                f(self.obj().unsafe_cast_ref::<ComboBox>().to_glib_none().0)
            }
        }
    }
    fn parent_changed(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkComboBoxClass;
            if let Some(f) = (*parent_class).changed {
                f(self.obj().unsafe_cast_ref::<ComboBox>().to_glib_none().0)
            }
        }
    }
    fn parent_format_entry_text(&self, path: &str) -> Option<GString> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkComboBoxClass;
            if let Some(f) = (*parent_class).format_entry_text {
                return Some(from_glib_full(f(
                    self.obj().unsafe_cast_ref::<ComboBox>().to_glib_none().0,
                    path.to_glib_none().0,
                )));
            }
            None
        }
    }
}

unsafe impl<T: ComboBoxImpl> IsSubclassable<T> for ComboBox {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.changed = Some(combo_box_changed::<T>);
        klass.format_entry_text = Some(combo_box_format_entry_text::<T>);
        #[cfg(any(feature = "v4_6", feature = "dox"))]
        {
            klass.activate = Some(combo_box_activate::<T>);
        };
    }
}

unsafe extern "C" fn combo_box_changed<T: ComboBoxImpl>(ptr: *mut ffi::GtkComboBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.changed()
}

unsafe extern "C" fn combo_box_format_entry_text<T: ComboBoxImpl>(
    ptr: *mut ffi::GtkComboBox,
    pathptr: *const libc::c_char,
) -> *mut libc::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let path: Borrowed<GString> = from_glib_borrow(pathptr);

    imp.format_entry_text(path.as_str()).into_glib_ptr()
}

#[cfg(any(feature = "v4_6", feature = "dox"))]
unsafe extern "C" fn combo_box_activate<T: ComboBoxImpl>(ptr: *mut ffi::GtkComboBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.activate()
}
