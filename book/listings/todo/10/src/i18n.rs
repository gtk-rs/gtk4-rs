use gettextrs::{
    LocaleCategory, bind_textdomain_codeset, bindtextdomain, setlocale, textdomain,
};

use crate::config;

// ANCHOR: init
pub fn init() {
    // Prepare i18n
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(config::gettext_package(), config::localedir())
        .expect("Unable to bind the text domain");
    bind_textdomain_codeset(config::gettext_package(), "UTF-8")
        .expect("Unable to set text domain encoding");
    textdomain(config::gettext_package()).expect("Unable to switch to the text domain");
}
// ANCHOR_END: init
