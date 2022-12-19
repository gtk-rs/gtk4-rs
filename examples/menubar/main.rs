use gtk::gio;
use gtk::prelude::*;

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
    let about = gio::ActionEntry::builder("about")
        .activate(|_, _, _| println!("About was pressed"))
        .build();

    let quit = gio::ActionEntry::builder("quit")
        .activate(|app: &gtk::Application, _, _| app.quit())
        .build();

    app.add_action_entries([about, quit]).unwrap();

    let menubar = {
        let file_menu = {
            let about_menu_item = gio::MenuItem::new(Some("About"), Some("app.about"));
            let quit_menu_item = gio::MenuItem::new(Some("Quit"), Some("app.quit"));

            let file_menu = gio::Menu::new();
            file_menu.append_item(&about_menu_item);
            file_menu.append_item(&quit_menu_item);
            file_menu
        };

        let menubar = gio::Menu::new();
        menubar.append_submenu(Some("File"), &file_menu);

        menubar
    };

    app.set_menubar(Some(&menubar));
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Menubar Example")
        .default_width(350)
        .default_height(70)
        .show_menubar(true)
        .build();

    window.show();
}
