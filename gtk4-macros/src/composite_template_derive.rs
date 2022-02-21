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
            klass.set_template_static(#template.as_bytes());
        },
    }
}

fn gen_template_child_bindings(fields: &[AttributedField]) -> TokenStream {
    let crate_ident = crate_ident_new();

    let recurse = fields.iter().map(|field| match field.attr.ty {
        FieldAttributeType::TemplateChild => {
            let mut value_id = &field.ident.to_string();
            let ident = &field.ident;
            let mut value_internal = false;
            field.attr.args.iter().for_each(|arg| match arg {
                FieldAttributeArg::Id(value) => {
                    value_id = value;
                }
                FieldAttributeArg::Internal(internal) => {
                    value_internal = *internal;
                }
            });

            quote! {
                klass.bind_template_child_with_offset(
                    &#value_id,
                    #value_internal,
                    #crate_ident::offset_of!(Self => #ident),
                );
            }
        }
    });

    quote! {
        #(#recurse)*
    }
}

fn gen_template_child_type_checks(fields: &[AttributedField]) -> TokenStream {
    let crate_ident = crate_ident_new();

    let recurse = fields.iter().map(|field| match field.attr.ty {
        FieldAttributeType::TemplateChild => {
            let ty = &field.ty;
            let ident = &field.ident;
            let type_err = format!("Template child with id `{}` has incompatible type. XML has {{:?}}, struct has {{:?}}", field.id());
            quote! {
                let ty = <<#ty as ::std::ops::Deref>::Target as #crate_ident::glib::StaticType>::static_type();
                let child_ty = #crate_ident::glib::object::ObjectExt::type_(::std::ops::Deref::deref(&imp.#ident));
                if !child_ty.is_a(ty) {
                    panic!(#type_err, child_ty, ty);
                }
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

    let source = match parse_template_source(input) {
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

    let attributed_fields = match parse_fields(fields) {
        Ok(fields) => fields,
        Err(err) => abort!(err.span(), err),
    };

    let template_children = gen_template_child_bindings(&attributed_fields);
    let checks = gen_template_child_type_checks(&attributed_fields);

    quote! {
        impl #crate_ident::subclass::widget::CompositeTemplate for #name {
            fn bind_template(klass: &mut Self::Class) {
                #set_template

                unsafe {
                    #template_children
                }
            }
            fn check_template_children(widget: &<Self as ObjectSubclass>::Type) {
                let imp = widget.imp();
                #checks
            }
        }
    }
}
