mod custom_button;

use gtk::prelude::*;
use gtk::{gdk, Application, ApplicationWindow, CssProvider, Orientation, gio, glib};

use custom_button::CustomButton;

const APP_ID: &str = "org.gtk_rs.Accessibility3";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("accessibility_3.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/org/gtk_rs/Accessibility3/style.css");
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let button1 = CustomButton::new("Click me");
    button1.connect_closure("activate", false, glib::closure_local!(|button: CustomButton| {
        println!("Button '{}' activated!", button.first_child().unwrap().property::<String>("label"));
    }));
    let button2 = CustomButton::new("Or me");
    button2.connect_closure("activate", false, glib::closure_local!(|button: CustomButton| {
        println!("Button '{}' activated!", button.first_child().unwrap().property::<String>("label"));
    }));

    let container = gtk::Box::new(Orientation::Vertical, 12);
    container.append(&button1);
    container.append(&button2);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Custom Button")
        .default_width(300)
        .default_height(200)
        .child(&container)
        .build();

    window.present();
}
