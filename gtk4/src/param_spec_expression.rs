// Take a look at the license at the top of the repository in the LICENSE file.

use glib::gobject_ffi;
use glib::translate::*;
use glib::ParamSpec;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GtkParamSpecExpression")]
    pub struct ParamSpecExpression(Shared<ffi::GtkParamSpecExpression>);

    match fn {
        ref => |ptr| gobject_ffi::g_param_spec_ref_sink(ptr as *mut gobject_ffi::GParamSpec),
        unref => |ptr| gobject_ffi::g_param_spec_unref(ptr as *mut gobject_ffi::GParamSpec),
        type_ => || ffi::gtk_param_expression_get_type(),
    }
}

#[allow(unknown_lints)]
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for ParamSpecExpression {}
unsafe impl Sync for ParamSpecExpression {}

impl std::ops::Deref for ParamSpecExpression {
    type Target = ParamSpec;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const ParamSpecExpression as *const ParamSpec) }
    }
}

unsafe impl glib::ParamSpecType for ParamSpecExpression {}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gobject_ffi::GParamSpec> for ParamSpecExpression {
    unsafe fn from_glib_full(ptr: *mut gobject_ffi::GParamSpec) -> Self {
        from_glib_full(ptr as *mut ffi::GtkParamSpecExpression)
    }
}

impl ParamSpecExpression {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "gtk_param_spec_expression")]
    pub fn new(name: &str, nick: &str, blurb: &str, flags: glib::ParamFlags) -> ParamSpec {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_param_spec_expression(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    pub fn upcast(self) -> ParamSpec {
        unsafe { from_glib_full(self.to_glib_full() as *mut gobject_ffi::GParamSpec) }
    }

    pub fn upcast_ref(&self) -> &ParamSpec {
        &*self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_synced;

    #[test]
    fn test_paramspec_expression() {
        test_synced(move || {
            let pspec = ParamSpecExpression::new(
                "expression",
                "Expression",
                "Some Expression",
                glib::ParamFlags::CONSTRUCT_ONLY | glib::ParamFlags::READABLE,
            );

            let expr_pspec = pspec.downcast::<ParamSpecExpression>();
            assert!(expr_pspec.is_ok());
        });
    }
}
