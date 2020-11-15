use anyhow::{bail, Result};
use itertools::Itertools;
use proc_macro2::{Ident, Span};
use proc_macro_crate::crate_name;
use syn::{Attribute, Lit, Meta, MetaList, NestedMeta};

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

#[derive(Debug)]
pub enum TemplateChildAttribute {
    Id(String),
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

fn parse_template_child_attribute(meta: &NestedMeta) -> Result<TemplateChildAttribute> {
    let (ident, v) = parse_attribute(meta)?;

    match ident.as_ref() {
        "id" => Ok(TemplateChildAttribute::Id(v)),
        s => bail!("Unknown item meta {}", s),
    }
}

pub fn parse_template_child_attributes(
    attr_name: &str,
    attrs: &[Attribute],
) -> Result<Vec<TemplateChildAttribute>> {
    let meta_list = find_attribute_meta(attrs, attr_name)?;
    let v = match meta_list {
        Some(meta) => meta
            .nested
            .iter()
            .map(|m| parse_template_child_attribute(&m))
            .fold_results(Vec::new(), |mut v, a| {
                v.push(a);
                v
            })?,
        None => Vec::new(),
    };

    Ok(v)
}

pub fn crate_ident_new() -> Ident {
    let crate_name = match crate_name("gtk4") {
        Ok(x) => x,
        Err(_) => "gtk4".to_owned(),
    };

    Ident::new(&crate_name, Span::call_site())
}
