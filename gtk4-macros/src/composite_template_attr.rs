use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::quote;

use std::env::var;
use std::path::PathBuf;
use std::string::ToString;

use crate::ui_definitions_parser;
use crate::ui_definitions_parser::UIDefinitions;
use crate::util::*;

fn read_template_file(file_path: &PathBuf, span: &Span) -> UIDefinitions {
    if let Some(file_path_str) = file_path.to_str() {
        match ui_definitions_parser::parse_file(&file_path) {
            Ok(template) => template,
            Err(err) => match err {
                ui_definitions_parser::Error::IOError(e) => {
                    abort!(
                        span,
                        "IO error when opening UI file `{}`: {}",
                        file_path_str,
                        e
                    );
                }
                ui_definitions_parser::Error::ParseError(e) => {
                    abort!(span, "Error parsing UI XML file `{}`: {}", file_path_str, e);
                }
            },
        }
    } else {
        abort!(span, "Can't convert the absolute filename to UTF8.");
    }
}

fn get_absolute_template_path(filename: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(&filename);

    if path.is_absolute() {
        Ok(path)
    } else {
        let manifest_dir = match var("CARGO_MANIFEST_DIR") {
            Ok(dir) => dir,
            Err(_) => {
                return Err("Path is not absolute and $CARGO_MANIFEST_DIR is not set.".to_string())
            }
        };

        let mut absolute_path = PathBuf::new();
        absolute_path.push(manifest_dir);
        absolute_path.push(&filename);
        Ok(absolute_path)
    }
}

/// Input is:
/// struct MyGtkSubclass {
/// }
///
/// We want to generate the struct members and the impl for
/// * CompositeTemplate
/// * ObjectSubclass
/// * ObjectImpl
pub fn gen_template(input: syn::ItemMod, attrs: syn::AttributeArgs) -> TokenStream {
    // This reduces the number of irrelevant error messages to be emitted when something is wrong
    proc_macro_error::set_dummy(quote!(
        #input
    ));

    let mut filename = String::new();
    let mut filename_span = Span::call_site();

    if let Ok(attrs) = parse_attribute_iter(&mut attrs.iter()) {
        attrs.into_iter().for_each(|a| match a {
            TemplateAttribute::Id(_, ident_span, _) => {
                abort!(ident_span, "Unkown template attribute 'id' for 'template'")
            }
            TemplateAttribute::Filename(name, _, value_span) => {
                filename = name;
                filename_span = value_span;
            }
        })
    };

    if filename.is_empty() {
        abort!(
            filename_span,
            "You have to specify a `file` attribute to the #[template] macro."
        )
    }

    let file_path = get_absolute_template_path(&filename)
        .unwrap_or_else(|err| abort!(filename_span, "{}", err));
    let file_path_str = file_path
        .to_str()
        .unwrap_or_else(|| abort!(filename_span, "Error converting filename to UTF8"));

    // Parse the template file. This can fail in multiple ways.
    let definitions = read_template_file(&file_path, &filename_span);

    let template = match definitions.template {
        Some(ref templ) => templ,
        None => abort!(
            filename_span,
            "The template file does not have a <template> attribute or the template \
            attribute is invalid"
        ),
    };

    let _parent_type = match generate_type_from_class_name(&template.parent) {
        Ok(typ) => typ,
        Err(err) => {
            abort_call_site!("Error parsing parent type of <template> tag: {}", err);
        }
    };

    let mod_attrs = input.attrs;
    let mod_ident = input.ident;
    let orig_content = input.content.map_or(Vec::new(), |tpl| tpl.1);
    let template_class = Ident::new(&template.klass, Span::call_site());
    let widget_class_str = template.klass.clone() + "Widgets";
    let widget_class = Ident::new(&widget_class_str, Span::call_site());
    let crate_ident = crate_ident_new();

    let mut field_idents = Vec::new();
    let mut fields = Vec::new();
    let mut child_bindings = Vec::new();

    for object in definitions.iter() {
        let typ = generate_type_from_class_name(&object.klass);
        match typ {
            Ok(typ) => {
                if let Some(name) = &object.id {
                    let name_ident = Ident::new(&name, Span::call_site());
                    field_idents.push(name_ident.clone());
                    fields.push(quote!(
                        pub #name_ident: TemplateChild<#typ>,
                    ));

                    let mut self_name = "Self::".to_string();
                    self_name.push_str(&name);

                    child_bindings.push(quote!(
                        klass.bind_template_child_with_offset(
                            &#name,
                            #crate_ident::offset_of!(Self => widgets) +
                                #crate_ident::offset_of!(#widget_class => #name_ident),
                        );
                    ));
                }
            }
            Err(err) => {
                abort_call_site!("Error parsing type: {}", err);
            }
        }
    }

    quote!(
        #(#mod_attrs)*
        mod #mod_ident {
            #(#orig_content)*

            #[derive(Debug)]
            pub struct #widget_class {
                #(#fields)*
            }

            impl CompositeTemplateWidgets for #widget_class {
                fn new() -> Self {
                    Self {
                        #(#field_idents: gtk::subclass::widget::TemplateChild::default()),*
                    }
                }
            }

            impl ImplicitCompositeTemplate for #template_class {
                fn bind_implicit_widgets(klass: &mut Self::Class) {
                    let template = include_bytes!(#file_path_str);
                    klass.set_template(template);

                    // Bind the regular template children if any
                    Self::bind_template_children(klass);

                    // Bind all implicit template children generated above
                    unsafe {
                        #(#child_bindings)*
                    }
                }
            }
        }
    )
}
