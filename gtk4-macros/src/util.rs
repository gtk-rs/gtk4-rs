// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::{Ident, Span};
use proc_macro_crate::crate_name;

pub fn crate_ident_new() -> Ident {
    let crate_name = match crate_name("gtk4") {
        Ok(x) => x,
        Err(_) => "gtk4".to_owned(),
    };

    Ident::new(&crate_name, Span::call_site())
}
