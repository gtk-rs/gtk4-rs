// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Buildable`](crate::Buildable) interface.

use crate::{prelude::*, subclass::prelude::*, Buildable, Builder};
use glib::translate::*;
use glib::{GString, Object, Quark, Value};
use once_cell::sync::Lazy;

use super::PtrHolder;

pub trait BuildableImpl: ObjectImpl {
    fn set_id(&self, id: &str) {
        self.parent_set_id(id)
    }
    fn id(&self) -> Option<GString> {
        self.parent_id()
    }
    fn add_child(&self, builder: &Builder, child: &Object, type_: Option<&str>) {
        self.parent_add_child(builder, child, type_)
    }
    fn set_buildable_property(&self, builder: &Builder, name: &str, value: &Value) {
        self.parent_set_buildable_property(builder, name, value)
    }
    fn parser_finished(&self, builder: &Builder) {
        self.parent_parser_finished(builder)
    }
    fn internal_child(&self, builder: &Builder, name: &str) -> Option<Object> {
        self.parent_internal_child(builder, name)
    }
    fn construct_child(&self, builder: &Builder, name: &str) -> Object {
        self.parent_construct_child(builder, name)
    }
    /*
    Only useful for custom tags, not something the application developer has to often implement
    and needs more thinking in terms of how to handle both BuildableParser & the various ptr you're supposed to pass around
    fn custom_tag_start(
        &self,
        builder: &Builder,
        child: Option<&Object>,
        tagname: &str,
        parser: BuildableParser,
        data: ptr,
    );
    fn custom_tag_end(
        &self,
        builder: &Builder,
        child: Option<&Object>,
        tagname: &str,
        data: ptr,
    );
    fn custom_finished(
        &self,
        builder: &Builder,
        child: Option<&Object>,
        tagname: &str,
        data: ptr,
    );
    */
}

pub trait BuildableImplExt: ObjectSubclass {
    fn parent_set_id(&self, id: &str);
    fn parent_id(&self) -> Option<GString>;
    fn parent_add_child(&self, builder: &Builder, child: &Object, type_: Option<&str>);
    fn parent_set_buildable_property(&self, builder: &Builder, name: &str, value: &Value);
    fn parent_parser_finished(&self, builder: &Builder);
    fn parent_internal_child(&self, builder: &Builder, name: &str) -> Option<Object>;
    fn parent_construct_child(&self, builder: &Builder, name: &str) -> Object;
}

impl<T: BuildableImpl> BuildableImplExt for T {
    fn parent_set_id(&self, id: &str) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            let func = (*parent_iface)
                .set_id
                .expect("no parent \"set_id\" implementation");

            func(
                self.obj().unsafe_cast_ref::<Buildable>().to_glib_none().0,
                id.to_glib_none().0,
            )
        }
    }

    fn parent_id(&self) -> Option<GString> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            let func = (*parent_iface)
                .get_id
                .expect("no parent \"get_id\" implementation");

            from_glib_none(func(
                self.obj().unsafe_cast_ref::<Buildable>().to_glib_none().0,
            ))
        }
    }

    fn parent_add_child(&self, builder: &Builder, child: &Object, type_: Option<&str>) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            let func = (*parent_iface)
                .add_child
                .expect("no parent \"add_child\" implementation");

            func(
                self.obj().unsafe_cast_ref::<Buildable>().to_glib_none().0,
                builder.to_glib_none().0,
                child.to_glib_none().0,
                type_.to_glib_none().0,
            )
        }
    }

    fn parent_set_buildable_property(&self, builder: &Builder, name: &str, value: &Value) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            // gtk::Builder falls back to using ObjectExt::set_property if the method is not implemented
            if let Some(func) = (*parent_iface).set_buildable_property {
                func(
                    self.obj().unsafe_cast_ref::<Buildable>().to_glib_none().0,
                    builder.to_glib_none().0,
                    name.to_glib_none().0,
                    value.to_glib_none().0,
                )
            } else {
                self.obj().set_property_from_value(name, value);
            }
        }
    }

    fn parent_parser_finished(&self, builder: &Builder) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            if let Some(func) = (*parent_iface).parser_finished {
                func(
                    self.obj().unsafe_cast_ref::<Buildable>().to_glib_none().0,
                    builder.to_glib_none().0,
                )
            }
        }
    }

    fn parent_internal_child(&self, builder: &Builder, name: &str) -> Option<Object> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            if let Some(func) = (*parent_iface).get_internal_child {
                from_glib_none(func(
                    self.obj().unsafe_cast_ref::<Buildable>().to_glib_none().0,
                    builder.to_glib_none().0,
                    name.to_glib_none().0,
                ))
            } else {
                None
            }
        }
    }

    fn parent_construct_child(&self, builder: &Builder, name: &str) -> Object {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            let func = (*parent_iface)
                .construct_child
                .expect("no parent \"construct_child\" implementation");

            from_glib_full(func(
                self.obj().unsafe_cast_ref::<Buildable>().to_glib_none().0,
                builder.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }
}

unsafe impl<T: BuildableImpl> IsImplementable<T> for Buildable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.set_id = Some(buildable_set_id::<T>);
        iface.get_id = Some(buildable_get_id::<T>);
        iface.add_child = Some(buildable_add_child::<T>);
        iface.set_buildable_property = Some(buildable_set_buildable_property::<T>);
        iface.construct_child = Some(buildable_construct_child::<T>);
        iface.parser_finished = Some(buildable_parser_finished::<T>);
        iface.get_internal_child = Some(buildable_get_internal_child::<T>);
        /* for the future
        iface.custom_tag_start = Some(buildable_custom_tag_start::<T>);
        iface.custom_tag_end = Some(buildable_custom_tag_end::<T>);
        iface.custom_finished = Some(buildable_custom_finished::<T>);
        */
    }
}

unsafe extern "C" fn buildable_set_id<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    id: *const libc::c_char,
) {
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();
    let id = from_glib_borrow::<_, GString>(id);

    imp.set_id(&id)
}

unsafe extern "C" fn buildable_get_id<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
) -> *const libc::c_char {
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();

    imp.id().into_glib_ptr()
}

unsafe extern "C" fn buildable_add_child<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    objectptr: *mut glib::gobject_ffi::GObject,
    typeptr: *const libc::c_char,
) {
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();
    let type_ = from_glib_borrow::<_, Option<GString>>(typeptr);

    imp.add_child(
        &from_glib_borrow(builderptr),
        &from_glib_borrow(objectptr),
        type_.as_ref().as_ref().map(|s| s.as_ref()),
    )
}

unsafe extern "C" fn buildable_set_buildable_property<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    nameptr: *const libc::c_char,
    valueptr: *const glib::gobject_ffi::GValue,
) {
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();
    let name = from_glib_borrow::<_, GString>(nameptr);

    imp.set_buildable_property(
        &from_glib_borrow(builderptr),
        &name,
        &from_glib_none(valueptr),
    )
}

unsafe extern "C" fn buildable_construct_child<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    nameptr: *const libc::c_char,
) -> *mut glib::gobject_ffi::GObject {
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();
    let name = from_glib_borrow::<_, GString>(nameptr);

    imp.construct_child(&from_glib_borrow(builderptr), &name)
        .into_glib_ptr()
}

unsafe extern "C" fn buildable_parser_finished<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
) {
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();

    imp.parser_finished(&from_glib_borrow(builderptr))
}

static BUILDABLE_GET_INTERNAL_CHILD_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_str("gtk4-rs-subclass-buildable-get-internal-child"));
unsafe extern "C" fn buildable_get_internal_child<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    nameptr: *const libc::c_char,
) -> *mut glib::gobject_ffi::GObject {
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();
    let name = from_glib_borrow::<_, GString>(nameptr);

    let ret = imp.internal_child(&from_glib_borrow(builderptr), &name);

    // transfer none: ensure the internal child stays alive for as long as the object building it
    let ret = ret.into_glib_ptr();
    imp.obj().set_qdata(
        *BUILDABLE_GET_INTERNAL_CHILD_QUARK,
        PtrHolder(ret, |ptr| {
            glib::gobject_ffi::g_object_unref(ptr as *mut _);
        }),
    );
    ret
}
