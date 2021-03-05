// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{DropDown, Expression, Widget};
use glib::translate::*;
use glib::Cast;

impl DropDown {
    #[doc(alias = "gtk_drop_down_new")]
    pub fn new<P: glib::IsA<gio::ListModel>, E: AsRef<Expression>>(
        model: Option<&P>,
        expression: Option<&E>,
    ) -> DropDown {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_drop_down_new(
                model.map(|p| p.as_ref()).to_glib_full(),
                expression.map(|e| e.as_ref()).to_glib_full(),
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_drop_down_set_expression")]
    pub fn set_expression<E: AsRef<Expression>>(&self, expression: Option<&E>) {
        unsafe {
            ffi::gtk_drop_down_set_expression(
                self.to_glib_none().0,
                expression.map(|e| e.as_ref()).to_glib_none().0,
            );
        }
    }
}
