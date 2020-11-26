use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;
/// This is an (incomplete) parser for .ui files
use xmlparser::{ElementEnd, Token, Tokenizer};

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ParseError(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::ParseError(e)
    }
}

#[derive(Debug, Clone)]
pub struct UITemplate {
    pub klass: String,
    pub parent: String,
    pub children: Vec<UIObject>,
}

#[derive(Debug, Clone)]
pub struct UIObject {
    pub klass: String,
    pub id: Option<String>,
    pub children: Vec<UIObject>,
}

#[derive(Debug, Clone)]
pub struct UIDefinitions {
    pub template: Option<UITemplate>,
    pub root_objects: Vec<UIObject>,
}

impl UIDefinitions {
    /// Returns a list of all objects contained in the interface
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &UIObject> + 'a> {
        fn flatten_objects_rec<'a>(
            objects: &'a [UIObject],
        ) -> Box<dyn Iterator<Item = &UIObject> + 'a> {
            Box::new(
                objects
                    .iter()
                    .map(|object| {
                        object
                            .children
                            .iter()
                            .chain(flatten_objects_rec(&object.children))
                    })
                    .flatten(),
            )
        }

        let mut iter = flatten_objects_rec(&self.root_objects);

        if let Some(template) = &self.template {
            iter = Box::new(
                iter.chain(
                    template
                        .children
                        .iter()
                        .chain(flatten_objects_rec(&template.children)),
                ),
            );
        }

        iter
    }
}

/// Parse the children attributes of an <object ..> or <template ..> element recursively
fn parse_children(
    mut tokenizer: &mut Tokenizer,
    element_token: &str,
) -> Result<Vec<UIObject>, String> {
    let mut children: Vec<UIObject> = Vec::new();

    while let Some(Ok(token)) = tokenizer.next() {
        match token {
            Token::ElementStart { local, .. } => {
                // <local
                if local.as_str().eq("object") {
                    // Parse <object> tag
                    // This returns after consuming the ElementEnd of the </object> element
                    let child_object = parse_object(&mut tokenizer)?;
                    children.push(child_object);
                } else if local.as_str().eq("template") {
                    return Err(
                        "<template ..> element must be a root element inside <interface ..>"
                            .to_string(),
                    );
                } else {
                    // Any other tag. We will enter it and see if we can extract any objects
                    // This is most likely a <children> or <child> tag, but anything would work
                    children.extend(parse_children(&mut tokenizer, local.as_str())?);
                }
            }
            Token::ElementEnd { end, .. } => {
                if let ElementEnd::Close(_, local) = end {
                    // </local>
                    if local.as_str().eq(element_token) {
                        break;
                    }
                };
            }
            _ => {}
        }
    }

    Ok(children)
}

/// Parse an <object ..> element and its children recursively
fn parse_object(mut tokenizer: &mut Tokenizer) -> Result<UIObject, String> {
    let mut klass: Option<String> = None;
    let mut id: Option<String> = None;
    let mut children: Vec<UIObject> = Vec::new();

    while let Some(Ok(token)) = tokenizer.next() {
        match token {
            Token::ElementEnd { end, .. } => {
                if let ElementEnd::Open = end {
                    // >
                    children = parse_children(&mut tokenizer, "object")?;
                }
            }
            Token::Attribute { local, value, .. } => {
                // local="value"
                let value = value.as_str().to_owned();

                if local.as_str().eq("id") {
                    id = Some(value);
                } else if local.as_str().eq("class") {
                    klass = Some(value);
                }
            }
            _ => {}
        }
    }

    if let Some(klass) = klass {
        return Ok(UIObject {
            klass,
            id,
            children,
        });
    }

    Err("<object ...> element must contain class attribute".to_string())
}

/// Parse a <template ..> element and it children recursively
fn parse_template(mut tokenizer: &mut Tokenizer) -> Result<UITemplate, String> {
    let mut klass: Option<String> = None;
    let mut parent: Option<String> = None;
    let mut children: Vec<UIObject> = Vec::new();

    while let Some(Ok(token)) = tokenizer.next() {
        match token {
            Token::ElementEnd { end, .. } => {
                match end {
                    ElementEnd::Open => {
                        // >
                        children = parse_children(&mut tokenizer, "template")?;
                    }
                    ElementEnd::Close(_, local) => {
                        // </local>
                        if local.as_str().eq("template") {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            Token::Attribute { local, value, .. } => {
                // local="value"
                let value = value.as_str().to_owned();

                if local.as_str().eq("parent") {
                    parent = Some(value);
                } else if local.as_str().eq("class") {
                    klass = Some(value);
                }
            }
            _ => {}
        }
    }

    if let Some(klass) = klass {
        if let Some(parent) = parent {
            return Ok(UITemplate {
                klass,
                parent,
                children,
            });
        }

        return Err("<template ...> element must contain parent attribute".to_string());
    }

    Err("<template ...> element must contain class attribute".to_string())
}

/// Parse the root <interface ..> element
fn parse_interface(mut tokenizer: &mut Tokenizer) -> Result<UIDefinitions, String> {
    let mut template: Option<UITemplate> = None;
    let mut root_objects: Vec<UIObject> = Vec::new();

    while let Some(token) = tokenizer.next() {
        if let Ok(Token::ElementStart { local, .. }) = token {
            // <local
            if local.as_str().eq("template") {
                if template.is_some() {
                    return Err("Only one <template ..> element allowed per UI file".to_string());
                }

                template = Some(parse_template(&mut tokenizer)?);
            } else if local.as_str().eq("object") {
                root_objects.push(parse_object(&mut tokenizer)?);
            }
        }
    }

    Ok(UIDefinitions {
        template,
        root_objects,
    })
}

/// Parse the UI Definitions from an XML string
pub fn parse_xml(xml: &str) -> Result<UIDefinitions, String> {
    let mut tokenizer = Tokenizer::from(xml);

    let mut interface: Option<UIDefinitions> = None;

    while let Some(token) = tokenizer.next() {
        if let Ok(Token::ElementStart { local, .. }) = token {
            // <local
            if local.as_str().eq("interface") {
                if interface.is_some() {
                    return Err(
                        "There must be only one <interface ..> element per UI Definitions file"
                            .to_string(),
                    );
                }
                interface = Some(parse_interface(&mut tokenizer)?);
            } else {
                return Err("The root element mut be <interface ..>".to_string());
            }
        }
    }

    if let Some(interface) = interface {
        return Ok(interface);
    }

    Err("No <interface> element found in UI file".to_string())
}

/// Parse the UI Definitions file from a filename
pub fn parse_file(filename: &str) -> Result<UIDefinitions, Error> {
    let mut file = File::open(&filename)?;

    let mut content_str = String::new();
    file.read_to_string(&mut content_str)?;

    Ok(parse_xml(&content_str)?)
}
