// Take a look at the license at the top of the repository in the LICENSE file.

use anyhow::{bail, Result};
use proc_macro2::Span;
use syn::{
    parse::Error, spanned::Spanned, Attribute, DeriveInput, Field, Fields, Ident, Lit, Meta,
    MetaList, MetaNameValue, NestedMeta, Type,
};

pub struct Template {
    pub source: TemplateSource,
    pub allow_template_child_without_attribute: bool,
}

pub enum TemplateSource {
    File(String),
    Resource(String),
    Xml(String),
    #[cfg(feature = "blueprint")]
    Blueprint(String),
}

impl TemplateSource {
    fn from_string_source(source: String) -> Result<Self> {
        for c in source.chars() {
            #[cfg(feature = "blueprint")]
            if c.is_ascii_alphabetic() {
                // blueprint code starts with some alphabetic letters
                return Ok(Self::Blueprint(source));
            } else if c == '<' {
                // xml tags starts with '<' symbol
                return Ok(Self::Xml(source));
            }
            #[cfg(not(feature = "blueprint"))]
            if c == '<' {
                // xml tags starts with '<' symbol
                return Ok(Self::Xml(source));
            }
        }
        bail!("Unknown lanuage")
    }
}

pub fn parse_template_source(input: &DeriveInput) -> Result<Template> {
    let meta = match find_attribute_meta(&input.attrs, "template")? {
        Some(meta) => meta,
        _ => bail!("Missing 'template' attribute"),
    };

    let mut allow_template_child_without_attribute = false;
    let mut source = None;

    for n in meta.nested.iter() {
        if let NestedMeta::Meta(m) = n {
            let p = m.path();
            if p.is_ident("file") || p.is_ident("resource") || p.is_ident("string") {
                if source.is_some() {
                    bail!(Error::new_spanned(
                        p,
                        "Specify only one of 'file', 'resource', or 'string'"
                    ));
                }
                source.replace(n);
            }
            if p.is_ident("allow_template_child_without_attribute") {
                if !matches!(m, Meta::Path(_)) {
                    bail!(Error::new_spanned(m, "Wrong meta"));
                }
                if allow_template_child_without_attribute {
                    bail!(Error::new_spanned(
                        p,
                        "Duplicate 'allow_template_child_without_attribute'"
                    ));
                }
                allow_template_child_without_attribute = true;
            }
        } else {
            bail!(Error::new_spanned(n, "wrong meta type"));
        }
    }
    let source = match source {
        Some(source) => source,
        None => bail!(Error::new_spanned(
            &meta,
            "Invalid meta, specify one of 'file', 'resource', or 'string'"
        )),
    };

    let (ident, v) = parse_attribute(source)?;

    let source = match ident.as_ref() {
        "file" => TemplateSource::File(v),
        "resource" => TemplateSource::Resource(v),
        "string" => TemplateSource::from_string_source(v)?,
        s => bail!("Unknown enum meta {}", s),
    };
    Ok(Template {
        source,
        allow_template_child_without_attribute,
    })
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
    Id(String, Span),
    Internal(bool),
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

impl AttributedField {
    pub fn id(&self) -> String {
        let mut name = None;
        for arg in &self.attr.args {
            if let FieldAttributeArg::Id(value, _) = arg {
                name = Some(value)
            }
        }
        name.cloned().unwrap_or_else(|| self.ident.to_string())
    }

    #[cfg(feature = "xml_validation")]
    pub fn id_span(&self) -> Span {
        for arg in &self.attr.args {
            if let FieldAttributeArg::Id(_, span) = arg {
                return *span;
            }
        }
        self.ident.span()
    }
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

fn parse_field_attr_value_bool(internal_value: &MetaNameValue) -> Result<bool, Error> {
    match &internal_value.lit {
        Lit::Bool(s) => Ok(s.value()),
        _ => Err(Error::new(
            internal_value.lit.span(),
            "invalid value type: Expected bool",
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
    let path = &name_value.path;
    let ident = path
        .get_ident()
        .ok_or_else(|| Error::new(path.span(), "invalid name type - expected identifier"))?;

    let ident_str = ident.to_string();
    let unknown_err = Err(Error::new(
        ident.span(),
        format!("unknown attribute argument: `{ident_str}`"),
    ));
    let value = match ty {
        FieldAttributeType::TemplateChild => match ident_str.as_str() {
            "id" => FieldAttributeArg::Id(
                parse_field_attr_value_str(name_value)?,
                name_value.lit.span(),
            ),
            "internal" => FieldAttributeArg::Internal(parse_field_attr_value_bool(name_value)?),
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
                let new_arg = parse_field_attr_meta(ty, meta)?;
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
            let args = parse_field_attr_args(&ty, field_attr)?;

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

fn path_is_template_child(path: &syn::Path) -> bool {
    if path.leading_colon.is_none()
        && path.segments.len() == 1
        && matches!(
            &path.segments[0].arguments,
            syn::PathArguments::AngleBracketed(_)
        )
        && path.segments[0].ident == "TemplateChild"
    {
        return true;
    }
    if path.segments.len() == 2
        && (path.segments[0].ident == "gtk" || path.segments[0].ident == "gtk4")
        && matches!(
            &path.segments[1].arguments,
            syn::PathArguments::AngleBracketed(_)
        )
        && path.segments[1].ident == "TemplateChild"
    {
        return true;
    }
    false
}

pub fn parse_fields(
    fields: &Fields,
    allow_missing_attribute: bool,
) -> Result<Vec<AttributedField>, Error> {
    let mut attributed_fields = Vec::new();

    for field in fields {
        let mut has_attr = false;
        if !field.attrs.is_empty() {
            if let Some(attributed_field) = parse_field(field)? {
                attributed_fields.push(attributed_field);
                has_attr = true;
            }
        }
        if !has_attr && !allow_missing_attribute {
            if let syn::Type::Path(syn::TypePath { path, .. }) = &field.ty {
                if path_is_template_child(path) {
                    proc_macro_error::emit_error!(
                        field,
                        "field `{}` with type `TemplateChild` possibly missing #[template_child] attribute. Use a meta attribute on the struct to suppress this error: '#[template(string|file|resource = \"...\", allow_template_child_without_attribute)]'",
                        field.ident.as_ref().unwrap().to_string(),
                    );
                }
            }
        }
    }

    Ok(attributed_fields)
}
