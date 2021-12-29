// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ObjectExpression;
use glib::translate::*;

define_expression!(ObjectExpression, ffi::GtkObjectExpression);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_synced;

    #[test]
    fn test_object_expression() {
        test_synced(move || {
            let obj = crate::IconTheme::new();
            let expr = ObjectExpression::new(&obj);
            assert_eq!(expr.object().unwrap(), obj);
        });
    }
}
