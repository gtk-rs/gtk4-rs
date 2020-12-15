// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn::Data;

use std::string::ToString;

use crate::attribute_parser::*;
use crate::util::*;

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

    let fields = match input.data {
        Data::Struct(ref s) => &s.fields,
        _ => abort_call_site!("derive(CompositeTemplate) only supports structs"),
    };

    let template_children = gen_template_child_bindings(&fields);

    quote! {
        impl #crate_ident::subclass::widget::CompositeTemplate for #name {
            fn bind_template_children(klass: &mut Self::Class) {
                unsafe {
                    #template_children
                }
            }
        }
    }
}
