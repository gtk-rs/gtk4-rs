// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Builder, BuilderClosureFlags, BuilderScope};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString};

pub trait BuilderScopeImpl: ObjectImpl {
    fn get_type_from_name(
        &self,
        builder_scope: &Self::Type,
        builder: &Builder,
        type_name: &str,
    ) -> glib::Type {
        unsafe {
            let type_ = ffi::gtk_builder_scope_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkBuilderScopeInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).get_type_from_name.as_ref().unwrap())(
                builder_scope
                    .unsafe_cast_ref::<BuilderScope>()
                    .to_glib_none()
                    .0,
                builder.to_glib_none().0,
                type_name.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib(ret)
        }
    }

    fn get_type_from_function(
        &self,
        builder_scope: &Self::Type,
        builder: &Builder,
        function_name: &str,
    ) -> glib::Type {
        unsafe {
            let type_ = ffi::gtk_builder_scope_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkBuilderScopeInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).get_type_from_function.as_ref().unwrap())(
                builder_scope
                    .unsafe_cast_ref::<BuilderScope>()
                    .to_glib_none()
                    .0,
                builder.to_glib_none().0,
                function_name.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);

            from_glib(ret)
        }
    }

    fn create_closure(
        &self,
        builder_scope: &Self::Type,
        builder: &Builder,
        function_name: &str,
        flags: BuilderClosureFlags,
        object: Option<&glib::Object>,
    ) -> Result<glib::Closure, glib::Error>;
}

unsafe impl<T: BuilderScopeImpl> IsImplementable<T> for BuilderScope {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let builder_scope_iface = &mut *(iface as *mut ffi::GtkBuilderScopeInterface);

        builder_scope_iface.get_type_from_name = Some(builder_scope_get_type_from_name::<T>);
        builder_scope_iface.get_type_from_function =
            Some(builder_scope_get_type_from_function::<T>);
        builder_scope_iface.create_closure = Some(builder_scope_create_closure::<T>);
    }
}

unsafe extern "C" fn builder_scope_get_type_from_name<T: BuilderScopeImpl>(
    builder_scope: *mut ffi::GtkBuilderScope,
    builderptr: *mut ffi::GtkBuilder,
    type_nameptr: *const libc::c_char,
) -> glib::ffi::GType {
    let instance = &*(builder_scope as *mut T::Instance);
    let imp = instance.get_impl();
    let builder: Borrowed<Builder> = from_glib_borrow(builderptr);
    let type_name: Borrowed<GString> = from_glib_borrow(type_nameptr);

    imp.get_type_from_name(
        from_glib_borrow::<_, BuilderScope>(builder_scope).unsafe_cast_ref(),
        &builder,
        &type_name,
    )
    .to_glib()
}

unsafe extern "C" fn builder_scope_get_type_from_function<T: BuilderScopeImpl>(
    builder_scope: *mut ffi::GtkBuilderScope,
    builderptr: *mut ffi::GtkBuilder,
    func_nameptr: *const libc::c_char,
) -> glib::ffi::GType {
    let instance = &*(builder_scope as *mut T::Instance);
    let imp = instance.get_impl();
    let builder: Borrowed<Builder> = from_glib_borrow(builderptr);
    let func_name: Borrowed<GString> = from_glib_borrow(func_nameptr);

    imp.get_type_from_function(
        from_glib_borrow::<_, BuilderScope>(builder_scope).unsafe_cast_ref(),
        &builder,
        &func_name,
    )
    .to_glib()
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
    let imp = instance.get_impl();
    let builder: Borrowed<Builder> = from_glib_borrow(builderptr);
    let func_name: Borrowed<GString> = from_glib_borrow(func_nameptr);
    let object: Borrowed<Option<glib::Object>> = from_glib_borrow(objectptr);

    let ret = imp.create_closure(
        from_glib_borrow::<_, BuilderScope>(builder_scope).unsafe_cast_ref(),
        &builder,
        &func_name,
        from_glib(flags),
        object.as_ref().as_ref(),
    );

    match ret {
        Ok(closure) => closure.to_glib_full(),
        Err(err) => {
            let mut err = std::mem::ManuallyDrop::new(err);
            *errorptr = err.to_glib_none_mut().0;
            std::ptr::null_mut()
        }
    }
}
