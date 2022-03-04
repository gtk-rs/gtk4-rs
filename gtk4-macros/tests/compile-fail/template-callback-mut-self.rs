// Take a look at the license at the top of the repository in the LICENSE file.

use gtk::glib;

mod imp {
    use super::*;
    use gtk::subclass::prelude::*;

    #[derive(Debug, Default)]
    pub struct MyWidget {}

    #[glib::object_subclass]
    impl ObjectSubclass for MyWidget {
        const NAME: &'static str = "MyWidget";
        type Type = super::MyWidget;
        type ParentType = gtk::Widget;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template_instance_callbacks();
        }
    }

    impl ObjectImpl for MyWidget {}
    impl WidgetImpl for MyWidget {}
}

glib::wrapper! {
    pub struct MyWidget(ObjectSubclass<imp::MyWidget>) @extends gtk::Widget;
}

#[gtk::template_callbacks]
impl MyWidget {
    #[template_callback]
    pub fn invalid_callback(&mut self) {}
}

fn main() {}
