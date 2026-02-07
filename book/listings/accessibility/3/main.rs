use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{AccessibleRole, Application, ApplicationWindow, GestureClick, Label, glib};

const APP_ID: &str = "org.gtk_rs.Accessibility3";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

// ANCHOR: custom_button
mod imp {
    use super::*;

    #[derive(Default)]
    pub struct CustomButton;

    #[glib::object_subclass]
    impl ObjectSubclass for CustomButton {
        const NAME: &'static str = "CustomButton";
        type Type = super::CustomButton;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            // Set the accessible role to Button
            klass.set_accessible_role(AccessibleRole::Button);
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }
    }

    impl ObjectImpl for CustomButton {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.set_focusable(true);
            obj.set_focus_on_click(true);
            obj.set_css_classes(&["card", "activatable"]);
        }

        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for CustomButton {}
}

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomButton {
    pub fn new(label: &str) -> Self {
        let obj: Self = glib::Object::builder().build();
        let child = Label::new(Some(label));
        child.set_parent(&obj);
        obj
    }
}

fn build_ui(app: &Application) {
    let custom_button = CustomButton::new("Click me");

    // Add click handling
    let gesture = GestureClick::new();
    gesture.connect_released(|_, _, _, _| {
        println!("Custom button clicked!");
    });
    custom_button.add_controller(gesture);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Custom Button")
        .default_width(300)
        .default_height(200)
        .child(&custom_button)
        .build();

    window.present();
}
// ANCHOR_END: custom_button
