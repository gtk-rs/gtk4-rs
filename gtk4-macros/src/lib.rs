mod composite_template_attr;
mod composite_template_derive;
mod ui_definitions_parser;
mod util;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemMod};

#[proc_macro_derive(CompositeTemplate, attributes(template_child, template_widgets))]
#[proc_macro_error]
pub fn composite_template_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = composite_template_derive::impl_composite_template(&input);
    gen.into()
}

/// This attribute macro is used to specify the template file.
/// It will generate a TemplateWidgets struct inside that is used to map the widgets specified
/// inside the template to the fields of the struct
///
/// # Arguments
///
/// * `file` - A string slice that holds the filename of the template. Required.
///
/// # Examples
///
/// ```
/// #[template(file="path/to/ui")]
/// mod my_widget_imp {
///     #[derive(CompositeTemplate)]
///     struct MyWidget {
///         #[templates]
///         widgets: TemplateWidgets,
///         other_fields,
///     }
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn template(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemMod);
    let attrs = parse_macro_input!(attrs as AttributeArgs);
    let gen = composite_template_attr::gen_template(item, attrs);
    gen.into()
}
