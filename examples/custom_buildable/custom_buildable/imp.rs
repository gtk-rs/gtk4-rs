use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

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
        klass.bind_template();

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
}
