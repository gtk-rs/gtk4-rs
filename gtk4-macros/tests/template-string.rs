// Take a look at the license at the top of the repository in the LICENSE file.

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(string = r#"
    <interface>
       <template class="MyWidget" parent="GtkWidget">
          <child>
            <object class="GtkLabel" id="label"/>
          </child>
       </template>
    </interface>
    "#)]
    pub struct MyWidget {
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MyWidget {
        const NAME: &'static str = "MyWidget";
        type Type = super::MyWidget;
        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MyWidget {}
    impl WidgetImpl for MyWidget {}
}

glib::wrapper! {
    pub struct MyWidget(ObjectSubclass<imp::MyWidget>) @extends gtk::Widget;
}

fn main() {
    gtk::init().unwrap();
    let _: MyWidget = glib::Object::new(&[]).unwrap();
}
