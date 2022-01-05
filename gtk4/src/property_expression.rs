// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PropertyExpression;
use glib::translate::*;

define_expression!(PropertyExpression, ffi::GtkPropertyExpression);

impl std::fmt::Debug for PropertyExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PropertyExpression")
            .field("value_type", &self.value_type())
            .field("is_static", &self.is_static())
            .field("pspec", &self.pspec())
            .field("expression", &self.expression())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_synced;
    use glib::StaticType;

    #[test]
    fn test_property_expression() {
        test_synced(move || {
            let _prop_expr = PropertyExpression::new(
                crate::StringObject::static_type(),
                crate::Expression::NONE,
                "string",
            );
        });
    }
}
