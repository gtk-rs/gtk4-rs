// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PropertyExpression;
use glib::translate::*;

define_expression!(PropertyExpression, ffi::GtkPropertyExpression);

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
