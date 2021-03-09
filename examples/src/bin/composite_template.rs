//! # Composite Template Example
//!
//! This sample demonstrates how to create a widget using GTK's composite templates.

use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::subclass::widget::WidgetImplExt;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;
    use gtk::subclass::prelude::*;

    /// The private struct, which can hold widgets and other data.
    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "composite_template.ui")]
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
        #[template_child]
        pub menubutton: TemplateChild<super::ExMenuButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExApplicationWindow {
        const NAME: &'static str = "ExApplicationWindow";
        type Type = super::ExApplicationWindow;
        type ParentType = gtk::ApplicationWindow;

        // Within class_init() you must set the template.
        // The CompositeTemplate derive macro provides a convenience function
        // bind_template() to set the template and bind all children at once.
        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
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

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "composite_template_child.ui")]
    pub struct ExMenuButton {
        #[template_child]
        pub toggle: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub popover: TemplateChild<gtk::Popover>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExMenuButton {
        const NAME: &'static str = "ExMenuButton";
        type Type = super::ExMenuButton;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExMenuButton {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            let popover = &*self.popover;
            self.toggle
                .connect_toggled(glib::clone!(@weak popover => move |toggle| {
                    if toggle.get_active() {
                        popover.popup();
                    }
                }));

            let toggle = &*self.toggle;
            self.popover
                .connect_closed(glib::clone!(@weak toggle => move |_| {
                    toggle.set_active(false);
                }));
        }

        // Needed for direct subclasses of GtkWidget;
        // Here you need to unparent all direct children
        // of your template.
        fn dispose(&self, obj: &Self::Type) {
            while let Some(child) = obj.get_first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for ExMenuButton {
        fn size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32) {
            self.parent_size_allocate(widget, width, height, baseline);
            self.popover.present();
        }
    }
}

glib::wrapper! {
    pub struct ExApplicationWindow(ObjectSubclass<imp::ExApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

glib::wrapper! {
    pub struct ExMenuButton(ObjectSubclass<imp::ExMenuButton>) @extends gtk::Widget;
}

impl ExApplicationWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create ExApplicationWindow")
    }

    pub fn init_label(&self) {
        // To access fields such as template children, you must get
        // the private struct.
        let self_ = imp::ExApplicationWindow::from_instance(self);
        self_
            .subtitle
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
