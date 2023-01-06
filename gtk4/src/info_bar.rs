// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{InfoBar, ResponseType};

impl InfoBar {
    #[doc(alias = "gtk_info_bar_new_with_buttons")]
    #[doc(alias = "new_with_buttons")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn with_buttons(buttons: &[(&str, ResponseType)]) -> Self {
        assert_initialized_main_thread!();
        let info_bar = InfoBar::new();
        info_bar.add_buttons(buttons);
        info_bar
    }

    #[doc(alias = "gtk_info_bar_add_buttons")]
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    pub fn add_buttons(&self, buttons: &[(&str, ResponseType)]) {
        for &(text, id) in buttons {
            self.add_button(text, id);
        }
    }
}
