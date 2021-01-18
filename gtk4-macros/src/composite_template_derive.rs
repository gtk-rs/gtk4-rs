// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn::Data;

use std::string::ToString;

use crate::attribute_parser::*;
use crate::util::*;

fn gen_set_template(source: TemplateSource) -> TokenStream {
    match source {
        TemplateSource::File(file) => quote! {
            let t = include_bytes!(#file);
            klass.set_template(t);
        },
        TemplateSource::Resource(resource) => quote! {
            klass.set_template_from_resource(&#resource);
        },
        TemplateSource::String(template) => quote! {
            klass.set_template(&#template);
        },
    }
}

fn gen_template_child_bindings(fields: &syn::Fields) -> TokenStream {
    let crate_ident = crate_ident_new();
    let attributed_fields = match parse_fields(&fields) {
        Ok(fields) => fields,
        Err(err) => abort!(err.span(), err.to_string()),
    };

    let recurse = attributed_fields.iter().map(|field| match field.attr.ty {
        FieldAttributeType::TemplateChild => {
            let mut value_id = &field.ident.to_string();
            let ident = &field.ident;
            field.attr.args.iter().for_each(|arg| match arg {
                FieldAttributeArg::Id(value) => {
                    value_id = &value;
                }
            });

            quote! {
                klass.bind_template_child_with_offset(
                    &#value_id,
                    #crate_ident::offset_of!(Self => #ident),
                );
            }
        }
    });

    quote! {
        #(#recurse)*
    }
}

pub fn impl_composite_template(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    let crate_ident = crate_ident_new();

    let source = match parse_template_source(&input) {
        Ok(v) => v,
        Err(e) => abort_call_site!(
            "{}: derive(CompositeTemplate) requires #[template(...)] to specify 'file', 'resource', or 'string'",
            e
        ),
    };

    let set_template = gen_set_template(source);

    let fields = match input.data {
        Data::Struct(ref s) => &s.fields,
        _ => abort_call_site!("derive(CompositeTemplate) only supports structs"),
    };

    let template_children = gen_template_child_bindings(&fields);

    quote! {
        impl #crate_ident::subclass::widget::CompositeTemplate for #name {
            fn bind_template(klass: &mut Self::Class) {
                #set_template

                unsafe {
                    #template_children
                }
            }
        }
    }
}
