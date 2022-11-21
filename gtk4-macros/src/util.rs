// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::{Ident, Span};
use proc_macro_crate::crate_name;

pub fn crate_ident_new() -> Ident {
    use proc_macro_crate::FoundCrate;

    let crate_name = match crate_name("gtk4").expect("missing gtk4 dependency in `Cargo.toml`") {
        FoundCrate::Name(name) => name,
        FoundCrate::Itself => "gtk4".to_owned(),
    };

    Ident::new(&crate_name, Span::call_site())
}
