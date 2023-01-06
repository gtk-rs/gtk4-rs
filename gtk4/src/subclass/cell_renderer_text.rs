// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`CellRendererText`](crate::CellRendererText).

use crate::{prelude::*, subclass::prelude::*, CellRendererText};
use glib::{translate::*, GString};

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait CellRendererTextImpl: CellRendererTextImplExt + CellRendererImpl {
    fn edited(&self, path: &str, new_text: &str) {
        self.parent_edited(path, new_text);
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait CellRendererTextImplExt: ObjectSubclass {
    fn parent_edited(&self, path: &str, new_text: &str);
}

impl<T: CellRendererTextImpl> CellRendererTextImplExt for T {
    fn parent_edited(&self, path: &str, new_text: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererTextClass;
            if let Some(f) = (*parent_class).edited {
                f(
                    self.obj()
                        .unsafe_cast_ref::<CellRendererText>()
                        .to_glib_none()
                        .0,
                    path.to_glib_none().0,
                    new_text.to_glib_none().0,
                )
            }
        }
    }
}

unsafe impl<T: CellRendererTextImpl> IsSubclassable<T> for CellRendererText {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.edited = Some(cell_renderer_text_edited::<T>);
    }
}

unsafe extern "C" fn cell_renderer_text_edited<T: CellRendererTextImpl>(
    ptr: *mut ffi::GtkCellRendererText,
    path: *const libc::c_char,
    new_text: *const libc::c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.edited(
        &GString::from_glib_borrow(path),
        &GString::from_glib_borrow(new_text),
    )
}
