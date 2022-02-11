// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Buildable`] interface.
use std::sync::OnceLock;

use glib::{GString, Object, Quark, Value, translate::*};

use crate::{Buildable, Builder, ffi, prelude::*, subclass::prelude::*};

use super::{BuildableParser, PtrHolder};

pub trait BuildableImpl: ObjectImpl + ObjectSubclass<Type: IsA<Buildable>> {
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
    fn custom_tag_start(
        &self,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
    ) -> Option<BuildableParser> {
        self.parent_custom_tag_start(builder, child, tag_name)
    }
    // rustdoc-stripper-ignore-next
    /// # Safety
    ///
    /// `data` must be the pointer originally returned by
    /// [`BuildableParser::new()`](BuildableParser::new) via
    /// [`custom_tag_start()`](BuildableImpl::custom_tag_start).
    unsafe fn custom_tag_end(
        &self,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        data: glib::ffi::gpointer,
    ) {
        unsafe { self.parent_custom_tag_end(builder, child, tag_name, data) }
    }
    // rustdoc-stripper-ignore-next
    /// # Safety
    ///
    /// `data` must be the pointer originally returned by
    /// [`BuildableParser::new()`](BuildableParser::new) via
    /// [`custom_tag_start()`](BuildableImpl::custom_tag_start).
    unsafe fn custom_finished(
        &self,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        data: glib::ffi::gpointer,
    ) {
        unsafe { self.parent_custom_finished(builder, child, tag_name, data) }
    }
}

pub trait BuildableImplExt: BuildableImpl {
    fn parent_set_id(&self, id: &str);
    fn parent_id(&self) -> Option<GString>;
    fn parent_add_child(&self, builder: &Builder, child: &Object, type_: Option<&str>);
    fn parent_set_buildable_property(&self, builder: &Builder, name: &str, value: &Value);
    fn parent_parser_finished(&self, builder: &Builder);
    fn parent_internal_child(&self, builder: &Builder, name: &str) -> Option<Object>;
    fn parent_construct_child(&self, builder: &Builder, name: &str) -> Object;
    fn parent_custom_tag_start(
        &self,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
    ) -> Option<BuildableParser>;
    // rustdoc-stripper-ignore-next
    /// # Safety
    ///
    /// `data` must be the pointer originally returned by
    /// [`BuildableParser::new()`](BuildableParser::new) via
    /// [`custom_tag_start()`](BuildableImpl::custom_tag_start).
    unsafe fn parent_custom_tag_end(
        &self,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        data: glib::ffi::gpointer,
    );
    // rustdoc-stripper-ignore-next
    /// # Safety
    ///
    /// `data` must be the pointer originally returned by
    /// [`BuildableParser::new()`](BuildableParser::new) via
    /// [`custom_tag_start()`](BuildableImpl::custom_tag_start).
    unsafe fn parent_custom_finished(
        &self,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        data: glib::ffi::gpointer,
    );
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

            // gtk::Builder falls back to using ObjectExt::set_property if the method is not
            // implemented
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

    fn parent_custom_tag_start(
        &self,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
    ) -> Option<BuildableParser> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            let func = (*parent_iface)
                .custom_tag_start
                .expect("no parent \"custom_tag_start\" implementation");

            let mut parser = std::mem::MaybeUninit::<ffi::GtkBuildableParser>::uninit();
            let mut data: glib::ffi::gpointer = std::ptr::null_mut();

            let ret: bool = from_glib(func(
                self.obj().unsafe_cast_ref::<Buildable>().to_glib_none().0,
                builder.to_glib_none().0,
                child.to_glib_none().0,
                tag_name.to_glib_none().0,
                parser.as_mut_ptr(),
                &mut data,
            ));
            if ret {
                Some(BuildableParser::from_raw_parts(parser.assume_init(), data))
            } else {
                None
            }
        }
    }

    unsafe fn parent_custom_tag_end(
        &self,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        data: glib::ffi::gpointer,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            let func = (*parent_iface)
                .custom_tag_end
                .expect("no parent \"custom_tag_end\" implementation");

            func(
                self.obj().unsafe_cast_ref::<Buildable>().to_glib_none().0,
                builder.to_glib_none().0,
                child.to_glib_none().0,
                tag_name.to_glib_none().0,
                data,
            )
        }
    }

    unsafe fn parent_custom_finished(
        &self,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        data: glib::ffi::gpointer,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            let func = (*parent_iface)
                .custom_finished
                .expect("no parent \"custom_finished\" implementation");

            func(
                self.obj().unsafe_cast_ref::<Buildable>().to_glib_none().0,
                builder.to_glib_none().0,
                child.to_glib_none().0,
                tag_name.to_glib_none().0,
                data,
            )
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
        iface.custom_tag_start = Some(buildable_custom_tag_start::<T>);
        iface.custom_tag_end = Some(buildable_custom_tag_end::<T>);
        iface.custom_finished = Some(buildable_custom_finished::<T>);
    }
}

unsafe extern "C" fn buildable_set_id<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    id: *const libc::c_char,
) {
    unsafe {
        let instance = &*(buildable as *mut T::Instance);
        let imp = instance.imp();
        let id = from_glib_borrow::<_, GString>(id);

        imp.set_id(&id)
    }
}

unsafe extern "C" fn buildable_get_id<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
) -> *const libc::c_char {
    unsafe {
        let instance = &*(buildable as *mut T::Instance);
        let imp = instance.imp();

        imp.id().into_glib_ptr()
    }
}

unsafe extern "C" fn buildable_add_child<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    objectptr: *mut glib::gobject_ffi::GObject,
    typeptr: *const libc::c_char,
) {
    unsafe {
        let instance = &*(buildable as *mut T::Instance);
        let imp = instance.imp();
        let type_ = from_glib_borrow::<_, Option<GString>>(typeptr);

        imp.add_child(
            &from_glib_borrow(builderptr),
            &from_glib_borrow(objectptr),
            type_.as_ref().as_ref().map(|s| s.as_ref()),
        )
    }
}

unsafe extern "C" fn buildable_set_buildable_property<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    nameptr: *const libc::c_char,
    valueptr: *const glib::gobject_ffi::GValue,
) {
    unsafe {
        let instance = &*(buildable as *mut T::Instance);
        let imp = instance.imp();
        let name = from_glib_borrow::<_, GString>(nameptr);

        imp.set_buildable_property(
            &from_glib_borrow(builderptr),
            &name,
            &from_glib_none(valueptr),
        )
    }
}

unsafe extern "C" fn buildable_construct_child<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    nameptr: *const libc::c_char,
) -> *mut glib::gobject_ffi::GObject {
    unsafe {
        let instance = &*(buildable as *mut T::Instance);
        let imp = instance.imp();
        let name = from_glib_borrow::<_, GString>(nameptr);

        imp.construct_child(&from_glib_borrow(builderptr), &name)
            .into_glib_ptr()
    }
}

unsafe extern "C" fn buildable_parser_finished<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
) {
    unsafe {
        let instance = &*(buildable as *mut T::Instance);
        let imp = instance.imp();

        imp.parser_finished(&from_glib_borrow(builderptr))
    }
}

unsafe extern "C" fn buildable_get_internal_child<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    nameptr: *const libc::c_char,
) -> *mut glib::gobject_ffi::GObject {
    unsafe {
        let instance = &*(buildable as *mut T::Instance);
        let imp = instance.imp();
        let name = from_glib_borrow::<_, GString>(nameptr);

        let ret = imp.internal_child(&from_glib_borrow(builderptr), &name);

        static QUARK: OnceLock<Quark> = OnceLock::new();
        let quark =
            *QUARK.get_or_init(|| Quark::from_str("gtk4-rs-subclass-buildable-get-internal-child"));

        // transfer none: ensure the internal child stays alive for as long as the
        // object building it
        let ret = ret.into_glib_ptr();
        imp.obj().set_qdata(
            quark,
            PtrHolder(ret, |ptr| {
                glib::gobject_ffi::g_object_unref(ptr as *mut _);
            }),
        );
        ret
    }
}

unsafe extern "C" fn buildable_custom_tag_start<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    child: *mut glib::gobject_ffi::GObject,
    nameptr: *const libc::c_char,
    parserptr: *mut ffi::GtkBuildableParser,
    data: *mut glib::ffi::gpointer,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(buildable as *mut T::Instance);
        let imp = instance.imp();
        let name = from_glib_borrow::<_, GString>(nameptr);
        let child = from_glib_borrow::<_, Option<Object>>(child);

        if let Some(parser) = imp.custom_tag_start(
            &from_glib_borrow(builderptr),
            child.as_ref().as_ref(),
            &name,
        ) {
            let (raw_parser, user_data) = parser.into_raw_parts();
            std::ptr::write(parserptr, raw_parser);
            *data = user_data;
            true.into_glib()
        } else {
            false.into_glib()
        }
    }
}

unsafe extern "C" fn buildable_custom_tag_end<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    child: *mut glib::gobject_ffi::GObject,
    nameptr: *const libc::c_char,
    data: glib::ffi::gpointer,
) {
    unsafe {
        let instance = &*(buildable as *mut T::Instance);
        let imp = instance.imp();
        let name = from_glib_borrow::<_, GString>(nameptr);
        let child = from_glib_borrow::<_, Option<Object>>(child);

        imp.custom_tag_end(
            &from_glib_borrow(builderptr),
            child.as_ref().as_ref(),
            &name,
            data,
        )
    }
}

unsafe extern "C" fn buildable_custom_finished<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    child: *mut glib::gobject_ffi::GObject,
    nameptr: *const libc::c_char,
    data: glib::ffi::gpointer,
) {
    unsafe {
        let instance = &*(buildable as *mut T::Instance);
        let imp = instance.imp();
        let name = from_glib_borrow::<_, GString>(nameptr);
        let child = from_glib_borrow::<_, Option<Object>>(child);

        imp.custom_finished(
            &from_glib_borrow(builderptr),
            child.as_ref().as_ref(),
            &name,
            data,
        )
    }
}
