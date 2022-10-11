use gtk::prelude::*;
use gtk::Application;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Run the application
    app.run();
}
