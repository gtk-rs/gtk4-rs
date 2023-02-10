// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Constraint, ConstraintLayout, Widget};
use glib::{translate::*, IntoStrV};

impl ConstraintLayout {
    #[doc(alias = "gtk_constraint_layout_add_constraints_from_descriptionv")]
    #[doc(alias = "gtk_constraint_layout_add_constraints_from_description")]
    #[doc(alias = "add_constraints_from_descriptionv")]
    pub fn add_constraints_from_description<'a, W: IsA<Widget>>(
        &self,
        lines: impl IntoStrV,
        hspacing: i32,
        vspacing: i32,
        views: impl IntoIterator<Item = (&'a str, &'a W)>,
    ) -> Result<Vec<Constraint>, glib::Error> {
        unsafe {
            let mut err = std::ptr::null_mut();
            let hash_table = glib::ffi::g_hash_table_new_full(
                Some(glib::ffi::g_str_hash),
                Some(glib::ffi::g_str_equal),
                Some(glib::ffi::g_free),
                Some(glib::ffi::g_free),
            );

            for (key, widget) in views {
                let key_ptr: *mut libc::c_char = key.to_glib_full();
                glib::ffi::g_hash_table_insert(
                    hash_table,
                    key_ptr as *mut _,
                    widget.to_glib_full() as *mut _,
                );
            }

            lines.run_with_strv(|lines| {
                let out = ffi::gtk_constraint_layout_add_constraints_from_descriptionv(
                    self.to_glib_none().0,
                    lines.as_ptr() as *const _,
                    lines.len().saturating_sub(1) as _,
                    hspacing,
                    vspacing,
                    hash_table,
                    &mut err,
                );
                if !err.is_null() {
                    Err(from_glib_full(err))
                } else {
                    Ok(FromGlibPtrContainer::from_glib_container(out))
                }
            })
        }
    }
}
