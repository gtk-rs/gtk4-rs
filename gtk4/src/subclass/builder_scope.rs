// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`BuilderScope`](crate::BuilderScope) interface.

use crate::{
    prelude::*, subclass::prelude::*, Builder, BuilderCScope, BuilderClosureFlags, BuilderScope,
};
use glib::{translate::*, GString};

pub trait BuilderCScopeImpl: BuilderScopeImpl {}

unsafe impl<T: BuilderCScopeImpl> IsSubclassable<T> for BuilderCScope {}

pub trait BuilderScopeImpl: ObjectImpl {
    #[doc(alias = "get_type_from_name")]
    fn type_from_name(&self, builder: &Builder, type_name: &str) -> glib::Type {
        self.parent_type_from_name(builder, type_name)
    }

    #[doc(alias = "get_type_from_function")]
    fn type_from_function(&self, builder: &Builder, function_name: &str) -> glib::Type {
        self.parent_type_from_function(builder, function_name)
    }

    fn create_closure(
        &self,
        builder: &Builder,
        function_name: &str,
        flags: BuilderClosureFlags,
        object: Option<&glib::Object>,
    ) -> Result<glib::Closure, glib::Error>;
}

pub trait BuilderScopeImplExt: ObjectSubclass {
    fn parent_type_from_name(&self, builder: &Builder, type_name: &str) -> glib::Type;

    fn parent_type_from_function(&self, builder: &Builder, function_name: &str) -> glib::Type;

    fn parent_create_closure(
        &self,
        builder: &Builder,
        function_name: &str,
        flags: BuilderClosureFlags,
        object: Option<&glib::Object>,
    ) -> Result<glib::Closure, glib::Error>;
}

impl<B: BuilderScopeImpl> BuilderScopeImplExt for B {
    fn parent_type_from_name(&self, builder: &Builder, type_name: &str) -> glib::Type {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<BuilderScope>()
                as *const ffi::GtkBuilderScopeInterface;

            let func = (*parent_iface)
                .get_type_from_name
                .expect("no parent \"get_type_from_name\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<BuilderScope>()
                    .to_glib_none()
                    .0,
                builder.to_glib_none().0,
                type_name.to_glib_none().0,
            ))
        }
    }

    fn parent_type_from_function(&self, builder: &Builder, function_name: &str) -> glib::Type {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<BuilderScope>()
                as *const ffi::GtkBuilderScopeInterface;

            let func = (*parent_iface)
                .get_type_from_function
                .expect("no parent \"get_type_from_function\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<BuilderScope>()
                    .to_glib_none()
                    .0,
                builder.to_glib_none().0,
                function_name.to_glib_none().0,
            ))
        }
    }

    fn parent_create_closure(
        &self,
        builder: &Builder,
        function_name: &str,
        flags: BuilderClosureFlags,
        object: Option<&glib::Object>,
    ) -> Result<glib::Closure, glib::Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<BuilderScope>()
                as *const ffi::GtkBuilderScopeInterface;

            let func = (*parent_iface)
                .create_closure
                .expect("no parent \"create_closure\" implementation");

            let mut error = std::ptr::null_mut();
            let closure = func(
                self.obj()
                    .unsafe_cast_ref::<BuilderScope>()
                    .to_glib_none()
                    .0,
                builder.to_glib_none().0,
                function_name.to_glib_none().0,
                flags.into_glib(),
                object.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(closure))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl<T: BuilderScopeImpl> IsImplementable<T> for BuilderScope {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        assert_initialized_main_thread!();

        iface.get_type_from_name = Some(builder_scope_get_type_from_name::<T>);
        iface.get_type_from_function = Some(builder_scope_get_type_from_function::<T>);
        iface.create_closure = Some(builder_scope_create_closure::<T>);
    }
}

unsafe extern "C" fn builder_scope_get_type_from_name<T: BuilderScopeImpl>(
    builder_scope: *mut ffi::GtkBuilderScope,
    builderptr: *mut ffi::GtkBuilder,
    type_nameptr: *const libc::c_char,
) -> glib::ffi::GType {
    let instance = &*(builder_scope as *mut T::Instance);
    let imp = instance.imp();
    let builder: Borrowed<Builder> = from_glib_borrow(builderptr);
    let type_name: Borrowed<GString> = from_glib_borrow(type_nameptr);

    imp.type_from_name(&builder, &type_name).into_glib()
}

unsafe extern "C" fn builder_scope_get_type_from_function<T: BuilderScopeImpl>(
    builder_scope: *mut ffi::GtkBuilderScope,
    builderptr: *mut ffi::GtkBuilder,
    func_nameptr: *const libc::c_char,
) -> glib::ffi::GType {
    let instance = &*(builder_scope as *mut T::Instance);
    let imp = instance.imp();
    let builder: Borrowed<Builder> = from_glib_borrow(builderptr);
    let func_name: Borrowed<GString> = from_glib_borrow(func_nameptr);

    imp.type_from_function(&builder, &func_name).into_glib()
}

unsafe extern "C" fn builder_scope_create_closure<T: BuilderScopeImpl>(
    builder_scope: *mut ffi::GtkBuilderScope,
    builderptr: *mut ffi::GtkBuilder,
    func_nameptr: *const libc::c_char,
    flags: ffi::GtkBuilderClosureFlags,
    objectptr: *mut glib::gobject_ffi::GObject,
    errorptr: *mut *mut glib::ffi::GError,
) -> *mut glib::gobject_ffi::GClosure {
    let instance = &*(builder_scope as *mut T::Instance);
    let imp = instance.imp();
    let builder: Borrowed<Builder> = from_glib_borrow(builderptr);
    let func_name: Borrowed<GString> = from_glib_borrow(func_nameptr);
    let object: Borrowed<Option<glib::Object>> = from_glib_borrow(objectptr);

    let ret = imp.create_closure(
        &builder,
        &func_name,
        from_glib(flags),
        object.as_ref().as_ref(),
    );

    match ret {
        Ok(closure) => closure.into_glib_ptr(),
        Err(e) => {
            *errorptr = e.into_glib_ptr();
            std::ptr::null_mut()
        }
    }
}
