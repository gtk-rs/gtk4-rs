use glib::translate::*;
use glib::GString;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use gtk::{
    prelude::*,
    subclass::{BuildableParser, BuildableParserImpl},
};
use std::os::raw::c_char;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "custom_buildable.ui")]
pub struct CustomBuildable {
    #[template_child]
    pub box_: TemplateChild<gtk::Box>,
    #[template_child]
    pub prefixes: TemplateChild<gtk::Box>,
    #[template_child]
    pub suffixes: TemplateChild<gtk::Box>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomBuildable {
    const NAME: &'static str = "CustomBuildable";
    type Type = super::CustomBuildable;
    type ParentType = gtk::Widget;
    type Interfaces = (gtk::Buildable,);

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);

        // The layout manager determines how child widgets are laid out.
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CustomBuildable {
    // Needed for direct subclasses of GtkWidget;
    // Here you need to unparent all direct children
    // of your template.
    fn dispose(&self, buildable: &Self::Type) {
        while let Some(child) = buildable.first_child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for CustomBuildable {}

impl BuildableImpl for CustomBuildable {
    fn add_child(
        &self,
        buildable: &Self::Type,
        builder: &gtk::Builder,
        child: &glib::Object,
        type_: Option<&str>,
    ) {
        // We first check if the main child `box_` has already been bound.
        if !self.box_.is_bound() {
            self.parent_add_child(buildable, builder, child, type_);
        } else if Some("prefix") == type_ {
            // Check if the child was added using `<child type="prefix">`
            buildable.add_prefix(child.downcast_ref::<gtk::Widget>().unwrap());
        } else if None == type_ {
            // Normal children
            buildable.add_suffix(child.downcast_ref::<gtk::Widget>().unwrap());
        };
    }

    unsafe fn custom_tag_start(
        &self,
        buildable: &Self::Type,
        builder: &gtk::Builder,
        child: Option<&glib::Object>,
        tag_name: &str,
        parser_data: *mut glib::ffi::gpointer,
    ) -> Option<BuildableParser> {
        if let Some(parser) =
            self.parent_custom_tag_start(buildable, builder, child, tag_name, parser_data)
        {
            return Some(parser);
        }

        let x = tag_name == "label-attribute";
        let parser = BuildableParser::new::<CustomBuildableParser>();
        *parser_data = &CustomBuildableParser::new() as *const _ as *mut _;
        Some(parser)
    }

    unsafe fn custom_tag_end(
        &self,
        buildable: &Self::Type,
        builder: &gtk::Builder,
        child: Option<&glib::Object>,
        tag_name: &str,
        parser_data: glib::ffi::gpointer,
    ) {
        let parser_data = parser_data as *const _ as *mut _ as CustomBuildableParser;
        println!("{:#?}", tag_name);
    }
}

#[repr(C)]
pub struct CustomBuildableParser {
    pub name: *const c_char,
}

impl CustomBuildableParser {
    pub fn new() -> Self {
        Self {
            name: std::ptr::null_mut(),
        }
    }
}

unsafe impl BuildableParserImpl for CustomBuildableParser {
    type Type = Self;
    fn start_element(
        parser: &mut Self::Type,
        ctx: &gtk::subclass::BuildableParseContext,
        element_name: &str,
        attributes: HashMap<String, String>,
    ) -> Result<(), glib::Error> {
        unsafe {
            parser.name = "hello world".as_ptr() as *const _;
        }
        println!("start element {}", element_name);
        println!("{:#?}", attributes);
        Ok(())
    }

    fn end_element(
        parser: &mut Self::Type,
        ctx: &gtk::subclass::BuildableParseContext,
        element_name: &str,
    ) -> Result<(), glib::Error> {
        unsafe {
            println!("{:#?}", from_glib_borrow::<_, Option<GString>>(parser.name));
        }
        println!("end element {}", element_name);
        Ok(())
    }

    fn text(
        parser: &mut Self::Type,
        ctx: &gtk::subclass::BuildableParseContext,
        text: &str,
    ) -> Result<(), glib::Error> {
        println!("text {}", text);

        Ok(())
    }
}
