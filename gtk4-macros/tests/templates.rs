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
          <object class="GtkLabel" id="label">
            <property name="label">foobar</property>
          </object>
        </child>
        <child>
          <object class="GtkLabel" id="my_label2">
            <property name="label">foobaz</property>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub struct MyWidget {
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        #[template_child(id = "my_label2")]
        pub label2: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MyWidget {
        const NAME: &'static str = "MyWidget";
        type Type = super::MyWidget;
        type ParentType = gtk::Widget;
        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MyWidget {
        fn dispose(&self, obj: &Self::Type) {
            while let Some(child) = obj.first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for MyWidget {}
}

glib::wrapper! {
    pub struct MyWidget(ObjectSubclass<imp::MyWidget>) @extends gtk::Widget;
}

#[gtk::test]
fn template_string() {
    let widget: MyWidget = glib::Object::new(&[]).unwrap();
    assert_eq!(widget.imp().label.label(), "foobar");
    assert_eq!(widget.imp().label2.label(), "foobaz");
}

mod imp2 {
    use super::*;
    use futures_channel::mpsc;
    use std::cell::RefCell;

    #[derive(Debug, CompositeTemplate)]
    #[template(string = r#"
    <interface>
       <template class="MyWidget2" parent="GtkWidget">
          <child>
            <object class="GtkButton" id="button">
                <signal name="clicked" handler="on_clicked" swapped="true"/>
            </object>
          </child>
       </template>
    </interface>
    "#)]
    pub struct MyWidget2 {
        #[template_child]
        button: TemplateChild<gtk::Button>,
        tx: RefCell<mpsc::Sender<u64>>,
        rx: RefCell<mpsc::Receiver<u64>>,
    }

    impl Default for MyWidget2 {
        fn default() -> Self {
            let (tx, rx) = mpsc::channel(1);
            Self {
                button: Default::default(),
                tx: RefCell::new(tx),
                rx: RefCell::new(rx),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MyWidget2 {
        const NAME: &'static str = "MyWidget2";
        type Type = super::MyWidget2;
        type ParentType = gtk::Widget;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    #[gtk::template_callbacks]
    impl MyWidget2 {
        #[template_callback]
        async fn on_clicked(&self, button: &gtk::Button) {
            glib::timeout_future_seconds(0).await;
            button.set_label("Clicked");
            self.tx.borrow_mut().try_send(42).unwrap();
        }
    }

    impl super::MyWidget2 {
        pub async fn click_button(&self) -> u64 {
            use futures_util::StreamExt;
            self.imp().button.emit_clicked();
            self.imp().rx.borrow_mut().next().await.unwrap()
        }
    }

    impl ObjectImpl for MyWidget2 {
        fn dispose(&self, _obj: &Self::Type) {
            self.button.unparent();
        }
    }
    impl WidgetImpl for MyWidget2 {}
}

glib::wrapper! {
    pub struct MyWidget2(ObjectSubclass<imp2::MyWidget2>) @extends gtk::Widget;
}

#[gtk::test]
async fn async_callbacks() {
    let widget: MyWidget2 = glib::Object::new(&[]).unwrap();
    assert_eq!(widget.click_button().await, 42);
}
