// Take a look at the license at the top of the repository in the LICENSE file.

use gtk::prelude::*;
use gtk::glib;

mod imp {
    use super::*;
    use gtk::subclass::prelude::*;
    use gtk::CompositeTemplate;

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
    #[allow(dead_code)]
    pub struct MyWidget {
        label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MyWidget {
        const NAME: &'static str = "MyWidget";
        type Type = super::MyWidget;
        type ParentType = gtk::Widget;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MyWidget {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for MyWidget {}
}

glib::wrapper! {
    pub struct MyWidget(ObjectSubclass<imp::MyWidget>) @extends gtk::Widget;
}

fn main() {}

