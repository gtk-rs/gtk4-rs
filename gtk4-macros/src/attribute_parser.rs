// Take a look at the license at the top of the repository in the LICENSE file.

use anyhow::{bail, Result};
use proc_macro2::Span;
use syn::parse::Error;
use syn::spanned::Spanned;
use syn::{
    Attribute, DeriveInput, Field, Fields, Ident, Lit, Meta, MetaList, MetaNameValue, NestedMeta,
    Type,
};

pub enum TemplateSource {
    File(String),
    Resource(String),
    String(String),
}

pub fn parse_template_source(input: &DeriveInput) -> Result<TemplateSource> {
    let meta = match find_attribute_meta(&input.attrs, "template")? {
        Some(meta) => meta,
        _ => bail!("Missing 'template' attribute"),
    };

    let meta = match meta.nested.iter().find(|n| match n {
        NestedMeta::Meta(m) => {
            let p = m.path();
            p.is_ident("file") || p.is_ident("resource") || p.is_ident("string")
        }
        _ => false,
    }) {
        Some(meta) => meta,
        _ => bail!("Invalid meta, specify one of 'file', 'resource', or 'string'"),
    };

    let (ident, v) = parse_attribute(meta)?;

    match ident.as_ref() {
        "file" => Ok(TemplateSource::File(v)),
        "resource" => Ok(TemplateSource::Resource(v)),
        "string" => Ok(TemplateSource::String(v)),
        s => bail!("Unknown enum meta {}", s),
    }
}

// find the #[@attr_name] attribute in @attrs
fn find_attribute_meta(attrs: &[Attribute], attr_name: &str) -> Result<Option<MetaList>> {
    let meta = match attrs.iter().find(|a| a.path.is_ident(attr_name)) {
        Some(a) => a.parse_meta(),
        _ => return Ok(None),
    };
    match meta? {
        Meta::List(n) => Ok(Some(n)),
        _ => bail!("wrong meta type"),
    }
}

// parse a single meta like: ident = "value"
fn parse_attribute(meta: &NestedMeta) -> Result<(String, String)> {
    let meta = match &meta {
        NestedMeta::Meta(m) => m,
        _ => bail!("wrong meta type"),
    };
    let meta = match meta {
        Meta::NameValue(n) => n,
        _ => bail!("wrong meta type"),
    };
    let value = match &meta.lit {
        Lit::Str(s) => s.value(),
        _ => bail!("wrong meta type"),
    };

    let ident = match meta.path.get_ident() {
        None => bail!("missing ident"),
        Some(ident) => ident,
    };

    Ok((ident.to_string(), value))
}

pub enum FieldAttributeArg {
    Id(String),
}

#[derive(Debug)]
pub enum FieldAttributeType {
    TemplateChild,
}

pub struct FieldAttribute {
    pub ty: FieldAttributeType,
    pub args: Vec<FieldAttributeArg>,
    pub path_span: Span,
    pub span: Span,
}

pub struct AttributedField {
    pub ident: Ident,
    pub ty: Type,
    pub attr: FieldAttribute,
}

fn parse_field_attr_value_str(name_value: &MetaNameValue) -> Result<String, Error> {
    match &name_value.lit {
        Lit::Str(s) => Ok(s.value()),
        _ => Err(Error::new(
            name_value.lit.span(),
            "invalid value type: Expected str literal",
        )),
    }
}

fn parse_field_attr_meta(
    ty: &FieldAttributeType,
    meta: &NestedMeta,
) -> Result<FieldAttributeArg, Error> {
    let meta = match &meta {
        NestedMeta::Meta(m) => m,
        _ => {
            return Err(Error::new(
                meta.span(),
                "invalid type - expected a name-value pair like id = \"widget\"",
            ))
        }
    };
    let name_value = match meta {
        Meta::NameValue(n) => n,
        _ => {
            return Err(Error::new(
                meta.span(),
                "invalid type - expected a name-value pair like id = \"widget\"",
            ))
        }
    };
    let ident = match name_value.path.get_ident() {
        None => {
            return Err(Error::new(
                name_value.path.span(),
                "invalid name type - expected identifier",
            ))
        }
        Some(ident) => ident,
    };

    let ident_str = ident.to_string();
    let unknown_err = Err(Error::new(
        ident.span(),
        &format!("unknown attribute argument: `{}`", ident_str),
    ));
    let value = match ty {
        FieldAttributeType::TemplateChild => match ident_str.as_str() {
            "id" => FieldAttributeArg::Id(parse_field_attr_value_str(&name_value)?),
            _ => return unknown_err,
        },
    };

    Ok(value)
}

fn parse_field_attr_args(
    ty: &FieldAttributeType,
    attr: &Attribute,
) -> Result<Vec<FieldAttributeArg>, Error> {
    let mut field_attribute_args = Vec::new();
    match attr.parse_meta()? {
        Meta::List(list) => {
            for meta in &list.nested {
                let new_arg = parse_field_attr_meta(ty, &meta)?;
                for arg in &field_attribute_args {
                    // Comparison of enum variants, not data
                    if std::mem::discriminant(arg) == std::mem::discriminant(&new_arg) {
                        return Err(Error::new(
                            meta.span(),
                            "two instances of the same attribute \
                            argument, each argument must be specified only once",
                        ));
                    }
                }
                field_attribute_args.push(new_arg);
            }
        }
        Meta::Path(_) => (),
        meta => {
            return Err(Error::new(
                meta.span(),
                "invalid attribute argument type, expected `name = value` list or nothing",
            ))
        }
    }

    Ok(field_attribute_args)
}

fn parse_field(field: &Field) -> Result<Option<AttributedField>, Error> {
    let field_attrs = &field.attrs;
    let ident = match &field.ident {
        Some(ident) => ident,
        None => return Err(Error::new(field.span(), "expected identifier")),
    };

    let ty = &field.ty;
    let mut attr = None;

    for field_attr in field_attrs {
        let span = field_attr.span();
        let path_span = field_attr.path.span();
        let ty = if field_attr.path.is_ident("template_child") {
            Some(FieldAttributeType::TemplateChild)
        } else {
            None
        };

        if let Some(ty) = ty {
            let args = parse_field_attr_args(&ty, &field_attr)?;

            if attr.is_none() {
                attr = Some(FieldAttribute {
                    ty,
                    args,
                    path_span,
                    span,
                })
            } else {
                return Err(Error::new(
                    span,
                    "multiple attributes on the same field are not supported",
                ));
            }
        }
    }

    if let Some(attr) = attr {
        Ok(Some(AttributedField {
            ident: ident.clone(),
            ty: ty.clone(),
            attr,
        }))
    } else {
        Ok(None)
    }
}

pub fn parse_fields(fields: &Fields) -> Result<Vec<AttributedField>, Error> {
    let mut attributed_fields = Vec::new();

    for field in fields {
        if !field.attrs.is_empty() {
            if let Some(attributed_field) = parse_field(field)? {
                attributed_fields.push(attributed_field)
            }
        }
    }

    Ok(attributed_fields)
}
