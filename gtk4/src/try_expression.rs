// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{Expression, TryExpression, ffi};

define_expression!(TryExpression, crate::ffi::GtkTryExpression);

impl TryExpression {
    #[doc(alias = "gtk_try_expression_new")]
    pub fn new(params: impl IntoIterator<Item = impl AsRef<Expression>>) -> Self {
        assert_initialized_main_thread!();

        let params = params
            .into_iter()
            .map(|e| e.as_ref().clone())
            .collect::<Vec<_>>();

        unsafe {
            from_glib_full(ffi::gtk_try_expression_new(
                params.len() as u32,
                params.to_glib_full(),
            ))
        }
    }
}
