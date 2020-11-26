use anyhow::{bail, Result};
use itertools::Itertools;
use proc_macro2::{Ident, Span};
use proc_macro_crate::crate_name;
use std::iter::{FromIterator, Iterator};
use syn::{Attribute, Lit, Meta, MetaList, NestedMeta, Type};

#[derive(Debug)]
pub enum TemplateAttribute {
    Id(String),
    Filename(String),
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

fn parse_attribute(meta: &NestedMeta) -> Result<(String, String)> {
    let meta = match &meta {
        NestedMeta::Meta(m) => m,
        _ => bail!("wrong meta type: not a NestedMeta::Meta"),
    };
    let meta = match meta {
        Meta::NameValue(n) => n,
        _ => bail!("wrong meta type: not a Meta::NameValue"),
    };
    let value = match &meta.lit {
        Lit::Str(s) => s.value(),
        _ => bail!("wrong meta type: not a Lit::Str"),
    };

    let ident = match meta.path.get_ident() {
        None => bail!("missing ident"),
        Some(ident) => ident,
    };

    Ok((ident.to_string(), value))
}

fn parse_template_attribute(meta: &NestedMeta) -> Result<TemplateAttribute> {
    let (ident, v) = parse_attribute(meta)?;

    match ident.as_ref() {
        "id" => Ok(TemplateAttribute::Id(v)),
        "file" => Ok(TemplateAttribute::Filename(v)),
        s => bail!("Unknown item meta {}", s),
    }
}

pub fn parse_attribute_iter(
    meta_list: &mut dyn Iterator<Item = &NestedMeta>,
) -> Result<Vec<TemplateAttribute>> {
    Ok(Vec::from_iter(
        meta_list
            .map(|m| parse_template_attribute(&m))
            .fold_results(Vec::new(), |mut v, a| {
                v.push(a);
                v
            })?,
    ))
}

pub fn parse_template_attributes(
    attr_name: &str,
    attrs: &[Attribute],
) -> Result<Vec<TemplateAttribute>> {
    let meta_list = find_attribute_meta(attrs, attr_name)?;
    let v = match meta_list {
        Some(meta) => parse_attribute_iter(&mut meta.nested.iter()),
        None => Ok(Vec::new()),
    };

    v
}

pub fn find_crate_name(name: &str) -> String {
    match crate_name(name) {
        Ok(x) => x,
        Err(_) => "gtk4".to_owned(),
    }
}

pub fn generate_type_from_class_name(class_name: &str) -> Result<Type, syn::Error> {
    let mut ident_name = class_name.to_string();

    if let Some(prefix) = class_name.get(0..3) {
        if let Some(suffix) = class_name.get(3..) {
            let mut lib = None;

            if prefix.eq("Gtk") {
                lib = Some("gtk4");
            } else if prefix.eq("Hdy") {
                lib = Some("libhandy");
            }

            if let Some(lib) = lib {
                ident_name = find_crate_name(lib);
                ident_name.push_str("::");
                ident_name.push_str(&suffix);
            }
        }
    }

    syn::parse_str(&ident_name)
}

pub fn crate_ident_new() -> Ident {
    Ident::new(&find_crate_name("gtk4"), Span::call_site())
}
