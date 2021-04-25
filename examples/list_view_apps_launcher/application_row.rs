use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;
    use gtk::CompositeTemplate;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "application_row.ui")]
    pub struct ApplicationRow {
        #[template_child]
        pub name: TemplateChild<gtk::Label>,
        #[template_child]
        pub description: TemplateChild<gtk::Label>,
        #[template_child]
        pub image: TemplateChild<gtk::Image>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ApplicationRow {
        const NAME: &'static str = "ApplicationRow";
        type Type = super::ApplicationRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ApplicationRow {}
    impl WidgetImpl for ApplicationRow {}
    impl BoxImpl for ApplicationRow {}
}

glib::wrapper! {
    pub struct ApplicationRow(ObjectSubclass<imp::ApplicationRow>)
        @extends gtk::Widget, gtk::Box;
}

impl Default for ApplicationRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationRow {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create ApplicationRow")
    }

    pub fn set_app_info(&self, app_info: &gio::AppInfo) {
        let self_ = imp::ApplicationRow::from_instance(self);
        self_.name.set_text(&app_info.name());
        if let Some(desc) = app_info.description() {
            self_.description.set_text(&desc);
        }
        if let Some(icon) = app_info.icon() {
            self_.image.set_from_gicon(&icon);
        }
    }
}
