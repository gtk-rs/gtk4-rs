// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Actionable};
use glib::{translate::*, Variant};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Actionable>> Sealed for T {}
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Actionable`](crate::Actionable).
pub trait ActionableExtManual: sealed::Sealed + IsA<Actionable> + 'static {
    #[doc(alias = "gtk_actionable_set_action_target")]
    #[doc(alias = "gtk_actionable_set_action_target_value")]
    fn set_action_target(&self, target: Option<impl Into<Variant>>) {
        unsafe {
            ffi::gtk_actionable_set_action_target_value(
                self.as_ref().to_glib_none().0,
                target.map(|v| v.into()).to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<Actionable>> ActionableExtManual for O {}
