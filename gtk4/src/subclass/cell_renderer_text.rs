// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString};

use super::cell_renderer::CellRendererImpl;
use crate::{CellRenderer, CellRendererText};

pub trait CellRendererTextImpl: CellRendererTextImplExt + CellRendererImpl {
    fn edited(&self, renderer: &Self::Type, path: &str, new_text: &str) {
        self.parent_edited(renderer, path, new_text);
    }
}

pub trait CellRendererTextImplExt: ObjectSubclass {
    fn parent_edited(&self, renderer: &Self::Type, path: &str, new_text: &str);
}

impl<T: CellRendererTextImpl> CellRendererTextImplExt for T {
    fn parent_edited(&self, renderer: &Self::Type, path: &str, new_text: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut ffi::GtkCellRendererTextClass;
            if let Some(f) = (*parent_class).edited {
                f(
                    renderer
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
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <CellRenderer as IsSubclassable<T>>::override_vfuncs(class);

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
    let imp = instance.get_impl();
    let wrap: Borrowed<CellRendererText> = from_glib_borrow(ptr);

    imp.edited(
        wrap.unsafe_cast_ref(),
        &GString::from_glib_borrow(path),
        &GString::from_glib_borrow(new_text),
    )
}
