//! # Composite Template Example
//!
//! This sample demonstrates how to create a widget using GTK's composite templates.

use gio::prelude::*;
use glib::subclass::prelude::*;
use glib::{glib_object_subclass, glib_wrapper};
use gtk::{prelude::*, CompositeTemplate};

mod imp {
    use super::*;
    use glib::subclass;
    use gtk::subclass::prelude::*;

    /// The private struct, which can hold widgets and other data.
    #[derive(Debug, CompositeTemplate)]
    pub struct ExApplicationWindow {
        // The #[template_child] attribute tells the CompositeTemplate macro
        // that a field is meant to be a child within the template.
        #[template_child]
        pub headerbar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        // You can specify the optional `id` argument if the id is not the same
        // as the identifier
        #[template_child(id = "subtitle_label")]
        pub subtitle: TemplateChild<gtk::Label>,
    }

    impl ObjectSubclass for ExApplicationWindow {
        const NAME: &'static str = "ExApplicationWindow";
        type Type = super::ExApplicationWindow;
        type ParentType = gtk::ApplicationWindow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        fn new() -> Self {
            Self {
                headerbar: TemplateChild::default(),
                label: TemplateChild::default(),
                subtitle: TemplateChild::default(),
            }
        }

        // Within class_init() you must set the template
        // and bind it's children. The CompositeTemplate
        // derive macro provides a convenience function
        // bind_template_children() to bind all children
        // at once.
        fn class_init(klass: &mut Self::Class) {
            let template = include_bytes!("composite_template.ui");
            klass.set_template(template);
            Self::bind_template_children(klass);
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExApplicationWindow {
        fn constructed(&self, obj: &Self::Type) {
            obj.init_label();
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for ExApplicationWindow {}
    impl WindowImpl for ExApplicationWindow {}
    impl ApplicationWindowImpl for ExApplicationWindow {}
}

glib_wrapper! {
    pub struct ExApplicationWindow(ObjectSubclass<imp::ExApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl ExApplicationWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(Self::static_type(), &[("application", app)])
            .expect("Failed to create ExApplicationWindow")
            .downcast::<ExApplicationWindow>()
            .expect("Created object is of wrong type")
    }

    pub fn init_label(&self) {
        // To access fields such as template children, you must get
        // the private struct.
        let self_ = imp::ExApplicationWindow::from_instance(self);
        self_
            .subtitle
            .get()
            .set_text("This is an example window made using composite templates");
    }
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.composite_template"),
        Default::default(),
    )
    .expect("Failed to initialize application");

    application.connect_activate(|app| {
        let win = ExApplicationWindow::new(app);
        win.show();
    });

    application.run(&std::env::args().collect::<Vec<_>>());
}
