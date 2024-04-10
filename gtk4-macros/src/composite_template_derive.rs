// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "xml_validation")]
use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
#[cfg(feature = "xml_validation")]
use quick_xml::name::QName;
use quote::quote;
use syn::{Data, Error, Result};

#[cfg(feature = "blueprint")]
use crate::blueprint::*;
use crate::{attribute_parser::*, util::*};

fn gen_set_template(source: &TemplateSource, crate_ident: &proc_macro2::Ident) -> TokenStream {
    match source {
        TemplateSource::File(file) => {
            let template = if file.ends_with(".blp") {
                if cfg!(feature = "blueprint") {
                    quote! {
                        #crate_ident::gtk4_macros::include_blueprint!(#file).as_bytes()
                    }
                } else {
                    panic!("blueprint feature is disabled")
                }
            } else {
                quote! {
                    include_bytes!(#file)
                }
            };

            quote! {
                #crate_ident::subclass::widget::WidgetClassExt::set_template_static(
                        klass,
                        #template,
                );
            }
        }
        TemplateSource::Resource(resource) => quote! {
            #crate_ident::subclass::widget::WidgetClassExt::set_template_from_resource(
                klass,
                &#resource,
            );
        },
        TemplateSource::Xml(template) => quote! {
            #crate_ident::subclass::widget::WidgetClassExt::set_template_static(
                klass,
                #template.as_bytes(),
            );
        },
        #[cfg(feature = "blueprint")]
        TemplateSource::Blueprint(blueprint) => {
            let template =
                compile_blueprint(blueprint.as_bytes()).expect("can't compile blueprint");

            quote! {
                #crate_ident::subclass::widget::WidgetClassExt::set_template_static(
                    klass,
                    #template.as_bytes(),
                );
            }
        }
    }
}

#[cfg(feature = "xml_validation")]
fn check_template_fields(source: &TemplateSource, fields: &[AttributedField]) -> Result<()> {
    #[allow(unused_assignments)]
    let xml = match source {
        TemplateSource::Xml(template) => template,
        _ => return Ok(()),
    };

    let mut reader = quick_xml::Reader::from_str(xml);
    let mut ids_left = fields
        .iter()
        .map(|field| match field.attr.ty {
            FieldAttributeType::TemplateChild => (field.id(), field.id_span()),
        })
        .collect::<HashMap<_, _>>();

    loop {
        use quick_xml::events::Event;

        let event = reader.read_event();
        let elem = match &event {
            Ok(Event::Start(e)) => Some(e),
            Ok(Event::Empty(e)) => Some(e),
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(Error::new(
                    Span::call_site(),
                    format!(
                        "Failed reading template XML at position {}: {:?}",
                        reader.buffer_position(),
                        e
                    ),
                ));
            }
            _ => None,
        };
        if let Some(e) = elem {
            let name = e.name();
            if name == QName(b"object") || name == QName(b"template") {
                let id = e
                    .attributes()
                    .find_map(|a| a.ok().and_then(|a| (a.key == QName(b"id")).then_some(a)));
                let id = id.as_ref().and_then(|a| std::str::from_utf8(&a.value).ok());
                if let Some(id) = id {
                    ids_left.remove(id);
                }
            }
        }
    }

    if let Some((name, span)) = ids_left.into_iter().next() {
        return Err(Error::new(
            span,
            format!("Template child with id `{name}` not found in template XML",),
        ));
    }

    Ok(())
}

fn gen_template_child_bindings(fields: &[AttributedField]) -> TokenStream {
    let crate_ident = crate_ident_new();

    let recurse = fields.iter().map(|field| match field.attr.ty {
        FieldAttributeType::TemplateChild => {
            let mut value_id = &field.ident.to_string();
            let ident = &field.ident;
            let mut value_internal = false;
            field.attr.args.iter().for_each(|arg| match arg {
                FieldAttributeArg::Id(value, _) => {
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
                let ty = <<#ty as ::std::ops::Deref>::Target as #crate_ident::glib::prelude::StaticType>::static_type();
                let child_ty = #crate_ident::glib::prelude::ObjectExt::type_(::std::ops::Deref::deref(&imp.#ident));
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

pub fn impl_composite_template(input: &syn::DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let crate_ident = crate_ident_new();

    let template = match parse_template_source(input) {
        Ok(v) => Some(v),
        Err(e) => {
            return Err(Error::new(
                Span::call_site(),
                format!("{}: derive(CompositeTemplate) requires #[template(...)] to specify 'file', 'resource', or 'string'",
                e)
            ));
        }
    };

    let allow_without_attribute = template
        .as_ref()
        .map(|t| t.allow_template_child_without_attribute)
        .unwrap_or(false);
    let source = template.as_ref().map(|t| &t.source);

    let set_template = source.map(|s| gen_set_template(s, &crate_ident));

    let fields = match input.data {
        Data::Struct(ref s) => Some(&s.fields),
        _ => {
            return Err(Error::new(
                Span::call_site(),
                "derive(CompositeTemplate) only supports structs",
            ));
        }
    };

    let attributed_fields = match fields.map(|f| parse_fields(f, allow_without_attribute)) {
        Some(fields) => fields?,
        None => vec![],
    };

    #[cfg(feature = "xml_validation")]
    {
        if let Some(source) = source {
            check_template_fields(source, &attributed_fields)?;
        }
    }
    let template_children = gen_template_child_bindings(&attributed_fields);
    let checks = gen_template_child_type_checks(&attributed_fields);

    Ok(quote! {
        impl #crate_ident::subclass::widget::CompositeTemplate for #name {
            fn bind_template(klass: &mut Self::Class) {
                #set_template

                unsafe {
                    #template_children
                }
            }
            fn check_template_children(widget: &<Self as #crate_ident::glib::subclass::prelude::ObjectSubclass>::Type) {
                let imp = #crate_ident::subclass::prelude::ObjectSubclassIsExt::imp(widget);
                #checks
            }
        }
    })
}
