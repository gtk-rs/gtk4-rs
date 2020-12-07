use proc_macro2::{Ident, Span};
use proc_macro_crate::crate_name;

pub fn crate_ident_new() -> Ident {
    let crate_name = match crate_name("gtk4") {
        Ok(x) => x,
        Err(_) => "gtk4".to_owned(),
    };

    Ident::new(&crate_name, Span::call_site())
}
