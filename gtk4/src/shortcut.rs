// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Shortcut, ShortcutAction, ShortcutTrigger};

impl Shortcut {
    #[doc(alias = "gtk_shortcut_new_with_arguments")]
    #[doc(alias = "new_with_arguments")]
    pub fn with_arguments(
        trigger: Option<impl IsA<ShortcutTrigger>>,
        action: Option<impl IsA<ShortcutAction>>,
        args: &glib::Variant,
    ) -> Self {
        assert_initialized_main_thread!();
        let shortcut = Shortcut::new(trigger, action);
        shortcut.set_arguments(Some(args));
        shortcut
    }
}
