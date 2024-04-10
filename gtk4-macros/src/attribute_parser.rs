// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, DeriveInput, Error, Field, Fields, Ident, LitBool, LitStr, Meta, Result, Token,
    Type,
};

/// Custom meta keywords.
mod kw {
    // `template` attribute.
    syn::custom_keyword!(file);
    syn::custom_keyword!(resource);
    syn::custom_keyword!(string);
    syn::custom_keyword!(allow_template_child_without_attribute);

    // `template_child` attribute.
    syn::custom_keyword!(id);
    syn::custom_keyword!(internal);
}

/// The parsed `template` attribute.
pub struct Template {
    pub source: TemplateSource,
    pub allow_template_child_without_attribute: bool,
}

impl Parse for Template {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut source = None;
        let mut allow_template_child_without_attribute = false;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::file) {
                let keyword: kw::file = input.parse()?;
                let _: Token![=] = input.parse()?;
                let value: LitStr = input.parse()?;

                if source.is_some() {
                    return Err(Error::new_spanned(
                        keyword,
                        "Specify only one of 'file', 'resource', or 'string'",
                    ));
                }

                source = Some(TemplateSource::File(value.value()));
            } else if lookahead.peek(kw::resource) {
                let keyword: kw::resource = input.parse()?;
                let _: Token![=] = input.parse()?;
                let value: LitStr = input.parse()?;

                if source.is_some() {
                    return Err(Error::new_spanned(
                        keyword,
                        "Specify only one of 'file', 'resource', or 'string'",
                    ));
                }

                source = Some(TemplateSource::Resource(value.value()));
            } else if lookahead.peek(kw::string) {
                let keyword: kw::string = input.parse()?;
                let _: Token![=] = input.parse()?;
                let value: LitStr = input.parse()?;

                if source.is_some() {
                    return Err(Error::new_spanned(
                        keyword,
                        "Specify only one of 'file', 'resource', or 'string'",
                    ));
                }

                source = Some(
                    TemplateSource::from_string_source(value.value())
                        .ok_or_else(|| Error::new_spanned(value, "Unknown language"))?,
                );
            } else if lookahead.peek(kw::allow_template_child_without_attribute) {
                let keyword: kw::allow_template_child_without_attribute = input.parse()?;

                if allow_template_child_without_attribute {
                    return Err(Error::new_spanned(
                        keyword,
                        "Duplicate 'allow_template_child_without_attribute'",
                    ));
                }

                allow_template_child_without_attribute = true;
            } else {
                return Err(lookahead.error());
            }

            if !input.is_empty() {
                let _: Token![,] = input.parse()?;
            }
        }

        let Some(source) = source else {
            return Err(Error::new(
                Span::call_site(),
                "Invalid meta, specify one of 'file', 'resource', or 'string'",
            ));
        };

        Ok(Template {
            source,
            allow_template_child_without_attribute,
        })
    }
}

/// The source of a template.
pub enum TemplateSource {
    File(String),
    Resource(String),
    Xml(String),
    #[cfg(feature = "blueprint")]
    Blueprint(String),
}

impl TemplateSource {
    fn from_string_source(value: String) -> Option<Self> {
        for c in value.chars() {
            #[cfg(feature = "blueprint")]
            if c.is_ascii_alphabetic() {
                // blueprint code starts with some alphabetic letters
                return Some(Self::Blueprint(value));
            } else if c == '<' {
                // xml tags starts with '<' symbol
                return Some(Self::Xml(value));
            }
            #[cfg(not(feature = "blueprint"))]
            if c == '<' {
                // xml tags starts with '<' symbol
                return Some(Self::Xml(value));
            }
        }

        None
    }
}

pub fn parse_template_source(input: &DeriveInput) -> Result<Template> {
    let Some(attr) = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("template"))
    else {
        return Err(Error::new(
            Span::call_site(),
            "Missing 'template' attribute",
        ));
    };

    attr.parse_args::<Template>()
}

/// An argument in a field attribute.
pub enum FieldAttributeArg {
    #[allow(dead_code)]
    // The span is needed for xml_validation feature
    Id(String, Span),
    Internal(bool),
}

impl FieldAttributeArg {
    fn from_template_child_meta(meta: &TemplateChildAttributeMeta) -> Self {
        match meta {
            TemplateChildAttributeMeta::Id { value, .. } => Self::Id(value.value(), value.span()),
            TemplateChildAttributeMeta::Internal { value, .. } => Self::Internal(value.value()),
        }
    }
}

/// The type of a field attribute.
#[derive(Debug)]
pub enum FieldAttributeType {
    TemplateChild,
}

/// A field attribute with args.
pub struct FieldAttribute {
    pub ty: FieldAttributeType,
    pub args: Vec<FieldAttributeArg>,
}

/// A field with an attribute.
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

/// A meta item of a `template_child` attribute.
enum TemplateChildAttributeMeta {
    Id {
        keyword: kw::id,
        value: LitStr,
    },
    Internal {
        keyword: kw::internal,
        value: LitBool,
    },
}

impl Parse for TemplateChildAttributeMeta {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::id) {
            let keyword = input.parse()?;
            let _: Token![=] = input.parse()?;
            let value = input.parse()?;
            Ok(Self::Id { keyword, value })
        } else if lookahead.peek(kw::internal) {
            let keyword = input.parse()?;
            let _: Token![=] = input.parse()?;
            let value = input.parse()?;
            Ok(Self::Internal { keyword, value })
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for TemplateChildAttributeMeta {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Id { keyword, .. } => keyword.to_tokens(tokens),
            Self::Internal { keyword, .. } => keyword.to_tokens(tokens),
        }
    }
}

fn parse_field_attr_args(ty: FieldAttributeType, attr: &Attribute) -> Result<FieldAttribute> {
    let mut args = Vec::new();

    if matches!(ty, FieldAttributeType::TemplateChild) && !matches!(attr.meta, Meta::Path(_)) {
        let meta_list = attr.parse_args_with(
            Punctuated::<TemplateChildAttributeMeta, Token![,]>::parse_terminated,
        )?;

        for meta in meta_list {
            let new_arg = FieldAttributeArg::from_template_child_meta(&meta);

            if args.iter().any(|arg| {
                // Comparison of enum variants, not data
                std::mem::discriminant(arg) == std::mem::discriminant(&new_arg)
            }) {
                return Err(Error::new_spanned(
                    meta,
                    "two instances of the same attribute \
                    argument, each argument must be specified only once",
                ));
            }

            args.push(new_arg);
        }
    }

    Ok(FieldAttribute { ty, args })
}

fn parse_field(field: &Field) -> Result<Option<AttributedField>> {
    let Some(ident) = &field.ident else {
        return Err(Error::new_spanned(field, "expected identifier"));
    };

    let mut attr_field = None;

    for attr in &field.attrs {
        let ty = if attr.path().is_ident("template_child") {
            FieldAttributeType::TemplateChild
        } else {
            continue;
        };

        let field_attr = parse_field_attr_args(ty, attr)?;

        if attr_field.is_some() {
            return Err(Error::new_spanned(
                attr,
                "multiple attributes on the same field are not supported",
            ));
        }

        attr_field = Some(AttributedField {
            ident: ident.clone(),
            ty: field.ty.clone(),
            attr: field_attr,
        })
    }

    Ok(attr_field)
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
) -> Result<Vec<AttributedField>> {
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
                    return Err(Error::new_spanned(
                        field,
                        format!("field `{}` with type `TemplateChild` possibly missing #[template_child] attribute. Use a meta attribute on the struct to suppress this error: '#[template(string|file|resource = \"...\", allow_template_child_without_attribute)]'",
                        field.ident.as_ref().unwrap())
                    ));
                }
            }
        }
    }

    Ok(attributed_fields)
}
