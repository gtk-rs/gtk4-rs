use std::cell::{Cell, RefCell};

use gtk::{glib, prelude::*, subclass::prelude::*};

#[derive(Default, glib::Properties)]
#[properties(wrapper_type = super::CustomEditable)]
pub struct CustomEditable {
    text: gtk::Text,
    pub spinner: RefCell<Option<gtk::Spinner>>,
    #[property(get, set = Self::set_show_spinner)]
    pub show_spinner: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomEditable {
    const NAME: &'static str = "CustomEditable";
    type Type = super::CustomEditable;
    type ParentType = gtk::Widget;
    type Interfaces = (gtk::Editable,);

    fn class_init(klass: &mut Self::Class) {
        // The call to `gtk_editable_install_properties` at `class_init` is
        // automatically done for you.
        klass.set_layout_manager_type::<gtk::BoxLayout>();
        klass.set_css_name("entry");
        klass.set_accessible_role(gtk::AccessibleRole::TextBox);
    }
}

impl ObjectImpl for CustomEditable {
    fn properties() -> &'static [glib::ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        // In case this is a property that's automatically added for Editable
        // implementations.
        if !self.delegate_set_property(id, value, pspec) {
            self.derived_set_property(id, value, pspec)
        }
    }

    fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        // In case this is a property that's automatically added for Editable
        // implementations.
        if let Some(value) = self.delegate_get_property(id, pspec) {
            value
        } else {
            self.derived_property(id, pspec)
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
        let editable = self.obj();
        // Most of the times when implementing Editable, you just want to embed
        // something like `gtk::Text` inside a more complex widget. In such
        // case, your implementation most forward the `gtk::Text`
        // or any `Editable` to the delegate. That starts by returning at
        // `EditableImpl::get_delegate`.
        //
        // In such case, the user has to initialize the delegate at `constructed` and
        // stop it at `dispose`. It mostly consists of wiring up/down some
        // signals from the delegate (`gtk::Text` in this case) to the internal editable
        // implementation.
        editable.init_delegate();
        self.text.set_hexpand(true);
        self.text.set_vexpand(true);

        self.text.set_parent(&*editable);
        editable.add_css_class("tagged");
        editable.set_enable_undo(true);
    }

    fn dispose(&self) {
        let editable = self.obj();

        // Wire down the delegate signals machinery
        editable.finish_delegate();
        self.text.unparent();
        while let Some(child) = editable.first_child() {
            child.unparent();
        }
    }
}
impl WidgetImpl for CustomEditable {
    fn grab_focus(&self) -> bool {
        self.text.grab_focus()
    }
}
impl EditableImpl for CustomEditable {
    fn delegate(&self) -> Option<gtk::Editable> {
        Some(self.text.clone().upcast())
    }
}

impl CustomEditable {
    pub fn set_show_spinner(&self, show_spinner: bool) {
        if self.show_spinner.get() == show_spinner {
            return;
        }

        if show_spinner {
            let spinner = gtk::Spinner::builder()
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .build();
            spinner.start();

            spinner.set_parent(self.obj().as_ref());
            self.spinner.replace(Some(spinner));
        } else if let Some(spinner) = self.spinner.borrow_mut().take() {
            spinner.unparent();
        }
        self.show_spinner.set(show_spinner);
    }
}
