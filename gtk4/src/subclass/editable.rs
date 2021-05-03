// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::Editable;
use glib::translate::*;
use glib::{Cast, GString, ObjectExt, Quark};
use libc::{c_char, c_int};
use once_cell::sync::Lazy;

pub trait EditableImpl: WidgetImpl {
    fn insert_text(&self, editable: &Self::Type, text: &str, length: i32, position: &mut i32) {
        self.parent_insert_text(editable, text, length, position);
    }

    fn delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32) {
        self.parent_delete_text(editable, start_position, end_position)
    }

    fn changed(&self, editable: &Self::Type) {
        self.parent_changed(editable)
    }

    #[doc(alias = "get_text")]
    fn text(&self, editable: &Self::Type) -> GString {
        self.parent_text(editable)
    }

    #[doc(alias = "get_delegate")]
    fn delegate(&self, editable: &Self::Type) -> Option<Editable> {
        self.parent_delegate(editable)
    }

    fn do_insert_text(&self, editable: &Self::Type, text: &str, length: i32, position: &mut i32) {
        self.parent_do_insert_text(editable, text, length, position)
    }

    fn do_delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32) {
        self.parent_do_delete_text(editable, start_position, end_position)
    }

    #[doc(alias = "get_selection_bounds")]
    fn selection_bounds(&self, editable: &Self::Type) -> Option<(i32, i32)> {
        self.parent_selection_bounds(editable)
    }

    fn set_selection_bounds(&self, editable: &Self::Type, start_position: i32, end_position: i32) {
        self.parent_set_selection_bounds(editable, start_position, end_position)
    }
}

pub trait EditableImplExt: ObjectSubclass {
    #[doc(alias = "gtk_editable_delegate_get_property")]
    fn delegate_get_property(
        &self,
        editable: &Self::Type,
        prop_id: usize,
        pspec: &glib::ParamSpec,
    ) -> Option<glib::Value> {
        unsafe {
            let mut value = glib::Value::from_type(pspec.value_type());

            if from_glib(ffi::gtk_editable_delegate_get_property(
                editable.unsafe_cast_ref::<glib::Object>().to_glib_none().0,
                prop_id as u32,
                value.to_glib_none_mut().0,
                pspec.to_glib_none().0,
            )) {
                Some(value)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_editable_delegate_set_property")]
    fn delegate_set_property(
        &self,
        editable: &Self::Type,
        prop_id: usize,
        value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_editable_delegate_set_property(
                editable.unsafe_cast_ref::<glib::Object>().to_glib_none().0,
                prop_id as u32,
                value.to_glib_none().0,
                pspec.to_glib_none().0,
            ))
        }
    }

    fn parent_insert_text(
        &self,
        editable: &Self::Type,
        text: &str,
        length: i32,
        position: &mut i32,
    );
    fn parent_delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32);
    fn parent_changed(&self, editable: &Self::Type);
    fn parent_do_insert_text(
        &self,
        editable: &Self::Type,
        text: &str,
        length: i32,
        position: &mut i32,
    );
    fn parent_do_delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32);
    fn parent_delegate(&self, editable: &Self::Type) -> Option<Editable>;
    fn parent_selection_bounds(&self, editable: &Self::Type) -> Option<(i32, i32)>;
    fn parent_set_selection_bounds(
        &self,
        editable: &Self::Type,
        start_position: i32,
        end_position: i32,
    );
    fn parent_text(&self, editable: &Self::Type) -> GString;
}

impl<T: EditableImpl> EditableImplExt for T {
    fn parent_insert_text(
        &self,
        editable: &Self::Type,
        text: &str,
        length: i32,
        position: &mut i32,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).insert_text {
                func(
                    editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
                    text.to_glib_none().0,
                    length,
                    position,
                );
            }
        }
    }

    fn parent_delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).delete_text {
                func(
                    editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
                    start_position,
                    end_position,
                );
            }
        }
    }

    fn parent_text(&self, editable: &Self::Type) -> GString {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;
            let func = (*parent_iface)
                .get_text
                .expect("no parent \"get_text\" implementation");

            from_glib_none(func(
                editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
            ))
        }
    }

    fn parent_delegate(&self, editable: &Self::Type) -> Option<Editable> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;
            let func = (*parent_iface)
                .get_delegate
                .expect("no parent \"get_delegate\" implementation");
            from_glib_none(func(
                editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
            ))
        }
    }

    fn parent_changed(&self, editable: &Self::Type) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).changed {
                func(editable.unsafe_cast_ref::<Editable>().to_glib_none().0);
            }
        }
    }

    fn parent_do_insert_text(
        &self,
        editable: &Self::Type,
        text: &str,
        length: i32,
        position: &mut i32,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).do_insert_text {
                func(
                    editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
                    text.to_glib_none().0,
                    length,
                    position,
                );
            }
        }
    }

    fn parent_do_delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).do_delete_text {
                func(
                    editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
                    start_position,
                    end_position,
                );
            }
        }
    }

    fn parent_selection_bounds(&self, editable: &Self::Type) -> Option<(i32, i32)> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).get_selection_bounds {
                let mut start_position = std::mem::MaybeUninit::uninit();
                let mut end_position = std::mem::MaybeUninit::uninit();
                if from_glib(func(
                    editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
                    start_position.as_mut_ptr(),
                    end_position.as_mut_ptr(),
                )) {
                    return Some((start_position.assume_init(), end_position.assume_init()));
                }
            }
            None
        }
    }

    fn parent_set_selection_bounds(
        &self,
        editable: &Self::Type,
        start_position: i32,
        end_position: i32,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Editable>()
                as *const ffi::GtkEditableInterface;

            if let Some(func) = (*parent_iface).set_selection_bounds {
                func(
                    editable.unsafe_cast_ref::<Editable>().to_glib_none().0,
                    start_position,
                    end_position,
                );
            }
        }
    }
}

unsafe impl<T: EditableImpl + ObjectSubclass> IsImplementable<T> for Editable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let instance_type = iface.instance_type();
        let iface = iface.as_mut();

        iface.insert_text = Some(editable_insert_text::<T>);
        iface.delete_text = Some(editable_delete_text::<T>);
        iface.changed = Some(editable_changed::<T>);
        iface.get_text = Some(editable_get_text::<T>);
        iface.get_delegate = Some(editable_get_delegate::<T>);
        iface.do_insert_text = Some(editable_do_insert_text::<T>);
        iface.do_delete_text = Some(editable_do_delete_text::<T>);
        iface.get_selection_bounds = Some(editable_get_selection_bounds::<T>);
        iface.set_selection_bounds = Some(editable_set_selection_bounds::<T>);

        unsafe {
            let class_ref = glib::object::Class::<glib::Object>::from_type(instance_type).unwrap();
            let object_class =
                class_ref.as_ref() as *const _ as *mut glib::gobject_ffi::GObjectClass;

            let mut first_prop = std::mem::MaybeUninit::uninit();
            let properties = glib::gobject_ffi::g_object_class_list_properties(
                object_class,
                first_prop.as_mut_ptr(),
            );
            glib::ffi::g_free(properties as *mut _);
            let first_prop = first_prop.assume_init();
            ffi::gtk_editable_install_properties(object_class, first_prop);
        }
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn editable_insert_text<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
    text: *const c_char,
    length: c_int,
    position: *mut c_int,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.impl_();

    imp.insert_text(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        &GString::from_glib_borrow(text),
        length,
        &mut *position,
    )
}

unsafe extern "C" fn editable_delete_text<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
    start_position: c_int,
    end_position: c_int,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.impl_();

    imp.delete_text(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        start_position,
        end_position,
    )
}

unsafe extern "C" fn editable_changed<T: EditableImpl>(editable: *mut ffi::GtkEditable) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.impl_();

    imp.changed(from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref())
}

unsafe extern "C" fn editable_get_text<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
) -> *const c_char {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.impl_();

    imp.text(from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref())
        .to_glib_full()
}

static EDITABLE_GET_DELEGATE_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-subclass-editable-get-delegate"));

unsafe extern "C" fn editable_get_delegate<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
) -> *mut ffi::GtkEditable {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.impl_();

    let wrap = from_glib_borrow::<_, Editable>(editable);

    let delegate = imp.delegate(wrap.unsafe_cast_ref());

    match wrap.qdata::<Option<Editable>>(*EDITABLE_GET_DELEGATE_QUARK) {
        Some(delegate_data) => {
            if delegate_data.as_ref() != &delegate {
                panic!("The Editable delegate must not change");
            }
        }
        None => {
            wrap.set_qdata(*EDITABLE_GET_DELEGATE_QUARK, delegate.clone());
        }
    };
    delegate.to_glib_none().0
}

unsafe extern "C" fn editable_do_insert_text<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
    text: *const c_char,
    length: i32,
    position: *mut i32,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.impl_();

    imp.do_insert_text(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        &GString::from_glib_borrow(text),
        length,
        &mut *position,
    )
}

unsafe extern "C" fn editable_do_delete_text<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
    start_position: i32,
    end_position: i32,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.impl_();

    imp.do_delete_text(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        start_position,
        end_position,
    )
}

unsafe extern "C" fn editable_get_selection_bounds<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
    start_position: *mut i32,
    end_position: *mut i32,
) -> glib::ffi::gboolean {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.impl_();

    if let Some((start_pos, end_pos)) =
        imp.selection_bounds(from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref())
    {
        if !start_position.is_null() {
            *start_position = start_pos;
        }

        if !end_position.is_null() {
            *end_position = end_pos;
        }
        true.into_glib()
    } else {
        *start_position = 0;
        *end_position = 0;
        false.into_glib()
    }
}

unsafe extern "C" fn editable_set_selection_bounds<T: EditableImpl>(
    editable: *mut ffi::GtkEditable,
    start_position: i32,
    end_position: i32,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.impl_();

    imp.set_selection_bounds(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        start_position,
        end_position,
    )
}
