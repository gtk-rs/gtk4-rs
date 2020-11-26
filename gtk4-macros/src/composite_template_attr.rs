use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;

use std::env::current_dir;
use std::path::Path;
use std::string::ToString;

use crate::ui_definitions_parser;
use crate::ui_definitions_parser::UIDefinitions;
use crate::util::*;

fn read_template_file(filename: String) -> UIDefinitions {
    if filename.is_empty() {
        abort_call_site!(&r#"Usage: #[template("NAME")]"#);
    }

    match ui_definitions_parser::parse_file(&filename) {
        Ok(template) => template,
        Err(err) => match err {
            ui_definitions_parser::Error::IOError(e) => {
                let mut abs = filename.to_string();
                let filename_path = Path::new(&filename);
                if !filename_path.is_absolute() {
                    if let Ok(working_dir) = current_dir() {
                        let working_dir = working_dir.join(&filename_path);
                        if let Some(working_dir) = working_dir.to_str() {
                            abs = working_dir.to_string();
                        }
                    }
                }
                abort_call_site!("IO error when opening UI file {}: {}", abs, e);
            }
            ui_definitions_parser::Error::ParseError(e) => {
                abort_call_site!("Error parsing UI XML file: {}", e);
            }
        },
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
pub fn gen_template(input: syn::ItemStruct, attrs: syn::AttributeArgs) -> TokenStream {
    let mut filename = String::new();

    if let Ok(attrs) = parse_attribute_iter(&mut attrs.iter()) {
        attrs.into_iter().for_each(|a| match a {
            TemplateAttribute::Id(_) => {
                abort_call_site!("Unkown template attribute 'id' for 'template'")
            }
            TemplateAttribute::Filename(name) => {
                filename = name;
            }
        })
    };

    if filename.is_empty() {
        abort_call_site!(
            "You have to specify a file=\"filename\" attribute to the #[template] macro."
        )
    }
    // Parse the template file. This can fail in multiple ways.
    let definitions = read_template_file(filename);

    let template = match definitions.template {
        Some(ref templ) => templ,
        None => abort_call_site!(
            "The template file does not have a <template> attribute or the template \
            attribute is invalid"
        ),
    };

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
                            #crate_ident::offset_of!(Self => #name_ident),
                        );
                    ));
                }
            }
            Err(err) => {
                abort_call_site!("Error parsing type: {}", err);
            }
        }
    }

    let parent_type = match generate_type_from_class_name(&template.parent) {
        Ok(typ) => typ,
        Err(err) => {
            abort_call_site!("Error parsing parent type of <template> tag: {}", err);
        }
    };

    //abort_call_site!("{:?}", fields.collect::<Vec<TokenTree>>());
    let struct_attrs = input.attrs;
    let vis = input.vis;
    let ident = input.ident;
    let ident_string = ident.to_string();
    let generics = input.generics;
    let orig_fields = input.fields.iter();

    let ident = quote!(#ident#generics);

    if !template.klass.eq(&ident_string) {
        abort_call_site!(
            "The struct is called {} but the UI Definitions expect a {}",
            ident_string,
            template.klass.to_string()
        );
    }

    quote!(
        #(#struct_attrs)*
        #vis struct #ident {
            #(#orig_fields,)*
            #(#fields)*
        }

        impl #crate_ident::subclass::widget::CompositeTemplate for #ident {
            fn bind_template_children(klass: &mut Self::Class) {
                unsafe {
                    #(#child_bindings)*
                }
            }
        }

        impl ObjectSubclass for #ident {
            const NAME: &'static str = #ident_string;
            type Type = super::#ident;
            type ParentType = #parent_type;
            type Instance = glib::subclass::simple::InstanceStruct<Self>;
            type Class = glib::subclass::simple::ClassStruct<Self>;

            glib_object_subclass!();

            fn new() -> Self {
                Self {
                    #(#field_idents: gtk::subclass::widget::TemplateChild::default()),*
                }
            }

            // Within class_init() you must set the template
            // and bind it's children. The CompositeTemplate
            // derive macro provides a convenience function
            // bind_template_children() to bind all children
            // at once.
            fn class_init(klass: &mut Self::Class) {
                let template = include_bytes!("composite_template.ui");
                klass.set_template(template);
                Self::bind_template_children(klass);
            }
        }

        impl ObjectImpl for #ident {
            fn constructed(&self, obj: &Self::Type) {
                obj.init_template();
                obj.init_label();
                self.parent_constructed(obj);
            }
        }

        /*impl #ident {
            pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
            glib::Object::new(Self::static_type(), &[("application", app)])
                .expect("Failed to create #ident_name")
                .downcast::<#ident>()
                .expect("Created object is of wrong type")
            }
        }*/
    )
}
