use std::cell::RefCell;
use std::collections::HashMap;

use gtk::{
    glib,
    prelude::*,
    subclass::{BuildableParser, BuildableParserImpl, prelude::*},
};

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "custom_buildable.ui")]
pub struct CustomBuildable {
    #[template_child]
    pub box_: TemplateChild<gtk::Box>,
    #[template_child]
    pub prefixes: TemplateChild<gtk::Box>,
    #[template_child]
    pub suffixes: TemplateChild<gtk::Box>,
    items: RefCell<Vec<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomBuildable {
    const NAME: &'static str = "CustomBuildable";
    type Type = super::CustomBuildable;
    type ParentType = gtk::Widget;
    type Interfaces = (gtk::Buildable,);

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();

        // The layout manager determines how child widgets are laid out.
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CustomBuildable {
    fn constructed(&self) {
        self.parent_constructed();

        let buildable = self.obj();
        for item in self.items.borrow().iter() {
            let label = gtk::Label::new(Some(item));
            label.add_css_class("dim-label");
            buildable.add_suffix(&label);
        }
    }

    // Needed for direct subclasses of GtkWidget;
    // Here you need to unparent all direct children
    // of your template.
    fn dispose(&self) {
        self.dispose_template();
    }
}

impl WidgetImpl for CustomBuildable {}

impl BuildableImpl for CustomBuildable {
    fn add_child(&self, builder: &gtk::Builder, child: &glib::Object, type_: Option<&str>) {
        let buildable = self.obj();
        // We first check if the main child `box_` has already been bound.
        if !self.box_.is_bound() {
            self.parent_add_child(builder, child, type_);
        } else if Some("prefix") == type_ {
            // Check if the child was added using `<child type="prefix">`
            buildable.add_prefix(child.downcast_ref::<gtk::Widget>().unwrap());
        } else if type_.is_none() {
            // Normal children
            buildable.add_suffix(child.downcast_ref::<gtk::Widget>().unwrap());
        };
    }

    fn custom_tag_start(
        &self,
        builder: &gtk::Builder,
        child: Option<&glib::Object>,
        tag_name: &str,
    ) -> Option<BuildableParser> {
        if let Some(parser) = self.parent_custom_tag_start(builder, child, tag_name) {
            return Some(parser);
        }

        if tag_name == "items" {
            Some(BuildableParser::new(ItemsParser::default()))
        } else {
            None
        }
    }

    unsafe fn custom_finished(
        &self,
        builder: &gtk::Builder,
        child: Option<&glib::Object>,
        tag_name: &str,
        data: glib::ffi::gpointer,
    ) {
        unsafe { self.parent_custom_finished(builder, child, tag_name, data) };

        if tag_name == "items" && !data.is_null() {
            let parser_data = unsafe { BuildableParser::take_data::<ItemsParser>(data) };
            *self.items.borrow_mut() = parser_data.items;
        }
    }
}

#[derive(Default)]
struct ItemsParser {
    items: Vec<String>,
    in_item: bool,
    current_text: String,
}

unsafe impl BuildableParserImpl for ItemsParser {
    fn start_element(
        &mut self,
        _ctx: &gtk::subclass::BuildableParseContext,
        element_name: &str,
        _attributes: HashMap<String, String>,
    ) -> Result<(), glib::Error> {
        if element_name == "item" {
            self.in_item = true;
            self.current_text.clear();
        }
        Ok(())
    }

    fn end_element(
        &mut self,
        _ctx: &gtk::subclass::BuildableParseContext,
        element_name: &str,
    ) -> Result<(), glib::Error> {
        if element_name == "item" {
            self.items.push(std::mem::take(&mut self.current_text));
            self.in_item = false;
        }
        Ok(())
    }

    fn text(
        &mut self,
        _ctx: &gtk::subclass::BuildableParseContext,
        text: &str,
    ) -> Result<(), glib::Error> {
        if self.in_item {
            self.current_text.push_str(text);
        }
        Ok(())
    }
}
