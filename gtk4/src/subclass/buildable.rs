// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Buildable`](crate::Buildable) interface.

use crate::prelude::*;
use crate::subclass::prelude::*;
use crate::{Buildable, Builder};
use glib::translate::*;
use glib::{Cast, GString, Object, Quark, Value};
use once_cell::sync::Lazy;

use super::{BuildableParser, PtrHolder};

pub trait BuildableImpl: ObjectImpl {
    fn set_id(&self, buildable: &Self::Type, id: &str) {
        self.parent_set_id(buildable, id)
    }
    fn id(&self, buildable: &Self::Type) -> Option<GString> {
        self.parent_id(buildable)
    }
    fn add_child(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        child: &Object,
        type_: Option<&str>,
    ) {
        self.parent_add_child(buildable, builder, child, type_)
    }
    fn set_buildable_property(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        name: &str,
        value: &Value,
    ) {
        self.parent_set_buildable_property(buildable, builder, name, value)
    }
    fn parser_finished(&self, buildable: &Self::Type, builder: &Builder) {
        self.parent_parser_finished(buildable, builder)
    }
    fn internal_child(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        name: &str,
    ) -> Option<Object> {
        self.parent_internal_child(buildable, builder, name)
    }
    fn construct_child(&self, buildable: &Self::Type, builder: &Builder, name: &str) -> Object {
        self.parent_construct_child(buildable, builder, name)
    }
    unsafe fn custom_tag_start(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        parser_data: *mut *mut libc::c_void,
    ) -> Option<BuildableParser> {
        self.parent_custom_tag_start(buildable, builder, child, tag_name, parser_data)
    }
    unsafe fn custom_tag_end(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        parser_data: *mut libc::c_void,
    ) {
        self.parent_custom_tag_end(buildable, builder, child, tag_name, parser_data)
    }
    unsafe fn custom_finished(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        parser_data: *mut libc::c_void,
    ) {
        self.parent_custom_finished(buildable, builder, child, tag_name, parser_data)
    }
}

pub trait BuildableImplExt: ObjectSubclass {
    fn parent_set_id(&self, buildable: &Self::Type, id: &str);
    fn parent_id(&self, buildable: &Self::Type) -> Option<GString>;
    fn parent_add_child(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        child: &Object,
        type_: Option<&str>,
    );
    fn parent_set_buildable_property(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        name: &str,
        value: &Value,
    );
    fn parent_parser_finished(&self, buildable: &Self::Type, builder: &Builder);
    fn parent_internal_child(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        name: &str,
    ) -> Option<Object>;
    fn parent_construct_child(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        name: &str,
    ) -> Object;
    unsafe fn parent_custom_tag_start(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        parser_data: *mut *mut libc::c_void,
    ) -> Option<BuildableParser>;
    unsafe fn parent_custom_tag_end(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        parser_data: *mut libc::c_void,
    );
    unsafe fn parent_custom_finished(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        parser_data: *mut libc::c_void,
    );
}

impl<T: BuildableImpl> BuildableImplExt for T {
    fn parent_set_id(&self, buildable: &Self::Type, id: &str) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            let func = (*parent_iface)
                .set_id
                .expect("no parent \"set_id\" implementation");

            func(
                buildable.unsafe_cast_ref::<Buildable>().to_glib_none().0,
                id.to_glib_none().0,
            )
        }
    }

    fn parent_id(&self, buildable: &Self::Type) -> Option<GString> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            let func = (*parent_iface)
                .get_id
                .expect("no parent \"get_id\" implementation");

            from_glib_none(func(
                buildable.unsafe_cast_ref::<Buildable>().to_glib_none().0,
            ))
        }
    }

    fn parent_add_child(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        child: &Object,
        type_: Option<&str>,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            let func = (*parent_iface)
                .add_child
                .expect("no parent \"add_child\" implementation");

            func(
                buildable.unsafe_cast_ref::<Buildable>().to_glib_none().0,
                builder.to_glib_none().0,
                child.to_glib_none().0,
                type_.to_glib_none().0,
            )
        }
    }

    fn parent_set_buildable_property(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        name: &str,
        value: &Value,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            // gtk::Builder falls back to using ObjectExt::set_property if the method is not implemented
            if let Some(func) = (*parent_iface).set_buildable_property {
                func(
                    buildable.unsafe_cast_ref::<Buildable>().to_glib_none().0,
                    builder.to_glib_none().0,
                    name.to_glib_none().0,
                    value.to_glib_none().0,
                )
            } else {
                buildable
                    .try_set_property_from_value(name, value)
                    .unwrap_or_else(|err| {
                        panic!("Buildable property `{}` was not found {}", name, err);
                    });
            }
        }
    }

    fn parent_parser_finished(&self, buildable: &Self::Type, builder: &Builder) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            if let Some(func) = (*parent_iface).parser_finished {
                func(
                    buildable.unsafe_cast_ref::<Buildable>().to_glib_none().0,
                    builder.to_glib_none().0,
                )
            }
        }
    }

    fn parent_internal_child(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        name: &str,
    ) -> Option<Object> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            if let Some(func) = (*parent_iface).get_internal_child {
                from_glib_none(func(
                    buildable.unsafe_cast_ref::<Buildable>().to_glib_none().0,
                    builder.to_glib_none().0,
                    name.to_glib_none().0,
                ))
            } else {
                None
            }
        }
    }

    fn parent_construct_child(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        name: &str,
    ) -> Object {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

            let func = (*parent_iface)
                .construct_child
                .expect("no parent \"construct_child\" implementation");

            from_glib_full(func(
                buildable.unsafe_cast_ref::<Buildable>().to_glib_none().0,
                builder.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    unsafe fn parent_custom_tag_start(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        parser_data: *mut *mut libc::c_void,
    ) -> Option<BuildableParser> {
        let type_data = Self::type_data();
        let parent_iface =
            type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

        let func = (*parent_iface)
            .custom_tag_start
            .expect("no parent \"custom_tag_start\" implementation");

        let parser = std::ptr::null_mut();
        let ret = from_glib(func(
            buildable.unsafe_cast_ref::<Buildable>().to_glib_none().0,
            builder.to_glib_none().0,
            child.to_glib_none().0,
            tag_name.to_glib_none().0,
            parser,
            parser_data,
        ));
        if ret {
            Some(BuildableParser::from_raw(parser))
        } else {
            None
        }
    }

    unsafe fn parent_custom_tag_end(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        parser_data: *mut libc::c_void,
    ) {
        let type_data = Self::type_data();
        let parent_iface =
            type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

        let func = (*parent_iface)
            .custom_tag_end
            .expect("no parent \"custom_tag_end\" implementation");

        func(
            buildable.unsafe_cast_ref::<Buildable>().to_glib_none().0,
            builder.to_glib_none().0,
            child.to_glib_none().0,
            tag_name.to_glib_none().0,
            parser_data,
        )
    }

    unsafe fn parent_custom_finished(
        &self,
        buildable: &Self::Type,
        builder: &Builder,
        child: Option<&Object>,
        tag_name: &str,
        parser_data: *mut libc::c_void,
    ) {
        let type_data = Self::type_data();
        let parent_iface =
            type_data.as_ref().parent_interface::<Buildable>() as *const ffi::GtkBuildableIface;

        let func = (*parent_iface)
            .custom_finished
            .expect("no parent \"custom_finished\" implementation");

        func(
            buildable.unsafe_cast_ref::<Buildable>().to_glib_none().0,
            builder.to_glib_none().0,
            child.to_glib_none().0,
            tag_name.to_glib_none().0,
            parser_data,
        )
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
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();
    let id = from_glib_borrow::<_, GString>(id);

    imp.set_id(
        from_glib_borrow::<_, Buildable>(buildable).unsafe_cast_ref(),
        &id,
    )
}

unsafe extern "C" fn buildable_get_id<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
) -> *const libc::c_char {
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();

    imp.id(from_glib_borrow::<_, Buildable>(buildable).unsafe_cast_ref())
        .to_glib_full()
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
        from_glib_borrow::<_, Buildable>(buildable).unsafe_cast_ref(),
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
        from_glib_borrow::<_, Buildable>(buildable).unsafe_cast_ref(),
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

    imp.construct_child(
        from_glib_borrow::<_, Buildable>(buildable).unsafe_cast_ref(),
        &from_glib_borrow(builderptr),
        &name,
    )
    .to_glib_full()
}

unsafe extern "C" fn buildable_parser_finished<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
) {
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();

    imp.parser_finished(
        from_glib_borrow::<_, Buildable>(buildable).unsafe_cast_ref(),
        &from_glib_borrow(builderptr),
    )
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
    let wrap = from_glib_borrow::<_, Buildable>(buildable);
    let name = from_glib_borrow::<_, GString>(nameptr);

    let ret = imp.internal_child(wrap.unsafe_cast_ref(), &from_glib_borrow(builderptr), &name);

    // transfer none: ensure the internal child stays alive for as long as the object building it
    let ret = ret.to_glib_full();
    wrap.set_qdata(
        *BUILDABLE_GET_INTERNAL_CHILD_QUARK,
        PtrHolder(ret, |ptr| {
            glib::gobject_ffi::g_object_unref(ptr as *mut _);
        }),
    );
    ret
}

unsafe extern "C" fn buildable_custom_tag_start<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    child: *mut glib::gobject_ffi::GObject,
    nameptr: *const libc::c_char,
    parserptr: *mut ffi::GtkBuildableParser,
    data: *mut glib::ffi::gpointer,
) -> glib::ffi::gboolean {
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();
    let wrap = from_glib_borrow::<_, Buildable>(buildable);
    let name = from_glib_borrow::<_, GString>(nameptr);
    let child = from_glib_borrow::<_, Option<Object>>(child);

    if let Some(parser) = imp.custom_tag_start(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(builderptr),
        child.as_ref().as_ref(),
        &name,
        data,
    ) {
        *parserptr = *parser.as_ptr();
        true.into_glib()
    } else {
        false.into_glib()
    }
}

unsafe extern "C" fn buildable_custom_tag_end<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    child: *mut glib::gobject_ffi::GObject,
    nameptr: *const libc::c_char,
    data: glib::ffi::gpointer,
) {
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();
    let wrap = from_glib_borrow::<_, Buildable>(buildable);
    let name = from_glib_borrow::<_, GString>(nameptr);
    let child = from_glib_borrow::<_, Option<Object>>(child);

    imp.custom_tag_end(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(builderptr),
        child.as_ref().as_ref(),
        &name,
        data,
    )
}

unsafe extern "C" fn buildable_custom_finished<T: BuildableImpl>(
    buildable: *mut ffi::GtkBuildable,
    builderptr: *mut ffi::GtkBuilder,
    child: *mut glib::gobject_ffi::GObject,
    nameptr: *const libc::c_char,
    data: glib::ffi::gpointer,
) {
    let instance = &*(buildable as *mut T::Instance);
    let imp = instance.imp();
    let wrap = from_glib_borrow::<_, Buildable>(buildable);
    let name = from_glib_borrow::<_, GString>(nameptr);
    let child = from_glib_borrow::<_, Option<Object>>(child);

    imp.custom_finished(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(builderptr),
        child.as_ref().as_ref(),
        &name,
        data,
    )
}
