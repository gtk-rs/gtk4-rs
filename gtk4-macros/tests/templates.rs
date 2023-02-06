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
        pub label2: gtk::TemplateChild<gtk::Label>,
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

#[gtk::test]
fn template_string() {
    let widget: MyWidget = glib::Object::new();
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
        rx: RefCell<Option<mpsc::Receiver<u64>>>,
    }

    impl Default for MyWidget2 {
        fn default() -> Self {
            let (tx, rx) = mpsc::channel(1);
            Self {
                button: Default::default(),
                tx: RefCell::new(tx),
                rx: RefCell::new(Some(rx)),
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
        pub async fn click_button(&self) -> Option<u64> {
            use futures_util::StreamExt;
            let mut rx = self.imp().rx.take()?;
            self.imp().button.emit_clicked();
            let v = rx.next().await?;
            self.imp().rx.replace(Some(rx));
            Some(v)
        }
    }

    impl ObjectImpl for MyWidget2 {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for MyWidget2 {}
}

glib::wrapper! {
    pub struct MyWidget2(ObjectSubclass<imp2::MyWidget2>) @extends gtk::Widget;
}

#[gtk::test]
async fn async_callbacks() {
    let widget: MyWidget2 = glib::Object::new();
    assert_eq!(widget.click_button().await, Some(42));
}

mod imp3 {
    use super::*;

    #[derive(Debug, Default)]
    struct TemplateChild<T>(T);

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(
        string = r#"
    <interface>
      <template class="MyWidget3" parent="GtkWidget"/>
    </interface>
    "#,
        allow_template_child_without_attribute
    )]
    pub struct MyWidget3 {
        _not_a_widget: TemplateChild<u32>,
    }

    #[glib::object_subclass]
    impl glib::subclass::prelude::ObjectSubclass for MyWidget3 {
        const NAME: &'static str = "MyWidget3";
        type Type = super::MyWidget3;
        type ParentType = gtk::Widget;
        fn class_init(klass: &mut Self::Class) {
            use gtk::subclass::widget::CompositeTemplateClass;
            klass.bind_template();
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl glib::subclass::prelude::ObjectImpl for MyWidget3 {}
    impl gtk::subclass::prelude::WidgetImpl for MyWidget3 {}
}

glib::wrapper! {
    pub struct MyWidget3(ObjectSubclass<imp3::MyWidget3>) @extends gtk::Widget;
}

#[gtk::test]
fn template_child_without_attribute() {
    let _: MyWidget3 = glib::Object::new();
}

#[cfg(feature = "blueprint")]
mod imp4 {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(string = "
    template MyWidget4 : Widget {
        Label label {
            label: 'foobar';
        }

        Label my_label2 {
            label: 'foobaz';
        }
    }
    ")]
    pub struct MyWidget4 {
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        #[template_child(id = "my_label2")]
        pub label2: gtk::TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MyWidget4 {
        const NAME: &'static str = "MyWidget4";
        type Type = super::MyWidget4;
        type ParentType = gtk::Widget;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MyWidget4 {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for MyWidget4 {}
}

#[cfg(feature = "blueprint")]
glib::wrapper! {
    pub struct MyWidget4(ObjectSubclass<imp4::MyWidget4>) @extends gtk::Widget;
}

#[gtk::test]
#[cfg(feature = "blueprint")]
fn blueprint_template() {
    let _: MyWidget4 = glib::Object::new();
}
