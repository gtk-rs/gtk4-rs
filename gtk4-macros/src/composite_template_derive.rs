use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::Data;

use std::string::ToString;

use crate::util::*;

fn gen_template_child_bindings(fields: &syn::Fields) -> TokenStream {
    let crate_ident = crate_ident_new();

    let recurse = fields.iter().map(|f| {
        let filtered_attrs = f
            .attrs
            .clone()
            .into_iter()
            .filter(|a| a.path.is_ident("template_child"))
            .collect::<Vec<syn::Attribute>>();
        if !filtered_attrs.is_empty() {
            let ident = f.ident.as_ref().unwrap();
            let mut value_id = String::new();

            if let Ok(attrs) = parse_template_child_attributes("template_child", &filtered_attrs) {
                attrs.into_iter().for_each(|a| match a {
                    TemplateChildAttribute::Id(id) => value_id = id,
                });
            }

            quote! {
                Self::bind_template_child_with_offset(
                    klass,
                    &#value_id,
                    #crate_ident::offset_of!(Self => #ident),
                );
            }
        } else {
            quote! {}
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
