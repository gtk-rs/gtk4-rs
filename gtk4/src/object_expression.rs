// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ObjectExpression;
use glib::translate::*;

define_expression!(ObjectExpression, ffi::GtkObjectExpression);

impl std::fmt::Debug for ObjectExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjectExpression")
            .field("value_type", &self.value_type())
            .field("is_static", &self.is_static())
            .field("object", &self.object())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as gtk4;

    #[test]
    fn test_object_expression() {
        let obj = crate::IconTheme::new();
        let expr = ObjectExpression::new(&obj);
        assert_eq!(expr.object().unwrap(), obj);
        assert!(expr.is::<ObjectExpression>());
    }
}
