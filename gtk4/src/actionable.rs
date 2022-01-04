// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Actionable;
use glib::translate::*;
use glib::{IsA, ToVariant};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Actionable`](crate::Actionable).
pub trait ActionableExtManual: 'static {
    #[doc(alias = "gtk_actionable_set_action_target")]
    #[doc(alias = "gtk_actionable_set_action_target_value")]
    fn set_action_target(&self, target: Option<&impl ToVariant>);
}

impl<O: IsA<Actionable>> ActionableExtManual for O {
    fn set_action_target(&self, target: Option<&impl ToVariant>) {
        unsafe {
            ffi::gtk_actionable_set_action_target_value(
                self.as_ref().to_glib_none().0,
                target.map(|v| v.to_variant()).to_glib_none().0,
            );
        }
    }
}
