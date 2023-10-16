use std::cell::RefCell;

use gtk::glib::{self, Properties};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Debug, Properties)]
#[properties(wrapper_type = super::CustomOrientable)]
pub struct CustomOrientable {
    first_label: RefCell<Option<gtk::Widget>>,
    second_label: RefCell<Option<gtk::Widget>>,
    // Every widget that implements Orientable has to define a "orientation"
    // property like below, gtk::Orientation::Horizontal is a placeholder
    // for the initial value.
    //
    // glib::ParamFlags::CONSTRUCT allows us to set that property the moment
    // we create a new instance of the widget
    #[property(get, set=Self::set_orientation, builder(gtk::Orientation::Horizontal))]
    orientation: RefCell<gtk::Orientation>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomOrientable {
    const NAME: &'static str = "ExCustomOrientable";
    type Type = super::CustomOrientable;
    type ParentType = gtk::Widget;
    type Interfaces = (gtk::Orientable,);

    fn class_init(klass: &mut Self::Class) {
        // The layout manager determines how child widgets are laid out.
        klass.set_layout_manager_type::<gtk::BoxLayout>();
    }

    fn new() -> Self {
        // Here we set the default orientation.
        Self {
            second_label: RefCell::new(None),
            first_label: RefCell::new(None),
            orientation: RefCell::new(gtk::Orientation::Horizontal),
        }
    }
}

impl ObjectImpl for CustomOrientable {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        // Create the children labels.
        let first_label = gtk::Label::new(Some("Hello"));
        let second_label = gtk::Label::new(Some("World!"));
        let layout_manager = obj
            .layout_manager()
            .and_downcast::<gtk::BoxLayout>()
            .unwrap();
        layout_manager.set_spacing(6);
        first_label.set_parent(&*obj);
        second_label.set_parent(&*obj);
        self.first_label
            .replace(Some(first_label.upcast::<gtk::Widget>()));
        self.second_label
            .replace(Some(second_label.upcast::<gtk::Widget>()));
    }

    fn dispose(&self) {
        // Child widgets need to be manually unparented in `dispose()`.
        if let Some(child) = self.first_label.borrow_mut().take() {
            child.unparent();
        }

        if let Some(child) = self.second_label.borrow_mut().take() {
            child.unparent();
        }
    }
}

impl WidgetImpl for CustomOrientable {}
impl OrientableImpl for CustomOrientable {}

impl CustomOrientable {
    pub fn set_orientation(&self, orientation: gtk::Orientation) {
        self.orientation.replace(orientation);
        // We have to set the value in our layout manager as well.
        let layout_manager = self
            .obj()
            .layout_manager()
            .and_downcast::<gtk::BoxLayout>()
            .unwrap();
        layout_manager.set_orientation(orientation);
    }
}
