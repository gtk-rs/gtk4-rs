// Take a look at the license at the top of the repository in the LICENSE file.

mod attribute_parser;
mod composite_template_derive;
mod util;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CompositeTemplate, attributes(template, template_child))]
#[proc_macro_error]
pub fn composite_template_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = composite_template_derive::impl_composite_template(&input);
    gen.into()
}
