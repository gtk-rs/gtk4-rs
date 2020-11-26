mod composite_template_attr;
mod composite_template_derive;
mod ui_definitions_parser;
mod util;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemStruct};

#[proc_macro_derive(CompositeTemplate, attributes(template_child))]
#[proc_macro_error]
pub fn composite_template_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = composite_template_derive::impl_composite_template(&input);
    gen.into()
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn template(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let attrs = parse_macro_input!(attrs as AttributeArgs);
    let gen = composite_template_attr::gen_template(item, attrs);
    gen.into()
}
