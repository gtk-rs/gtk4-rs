use gtk::prelude::*;
use gtk::{gio, glib};

fn main() {
    // Create a new application
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.menubar"),
        Default::default(),
    );
    application.connect_startup(configure_application);
    application.connect_activate(build_ui);
    application.run();
}

fn configure_application(app: &gtk::Application) {
    let about = gio::SimpleAction::new("about", None);
    about.connect_activate(|_, _| {
        println!("About was pressed");
    });
    app.add_action(&about);

    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(glib::clone!(@weak app => move |_action, _parameter| {
        app.quit();
    }));
    app.add_action(&quit);

    // About menu item
    let about_menu = gio::MenuItem::new(Some("About"), Some("app.about"));

    // Quit menu item
    let quit_menu = gio::MenuItem::new(Some("Quit"), Some("app.quit"));

    // File Menu
    let file_menu = gio::Menu::new();
    file_menu.append_item(&about_menu);
    file_menu.append_item(&quit_menu);

    // Menubar
    let menubar = gio::Menu::new();
    menubar.append_submenu(Some("File"), &file_menu);

    // Put menubar into application
    app.set_menubar(Some(&menubar));
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("Menubar Example"));
    window.set_default_size(350, 70);

    window.set_show_menubar(true);

    window.show();
}
