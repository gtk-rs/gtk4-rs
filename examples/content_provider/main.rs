mod content_provider;

use content_provider::ContentProvider;
use gtk::glib;
use gtk::prelude::*;

fn main() {
    let app = gtk::Application::new(Some("org.gtk.content-provider"), Default::default());
    app.connect_activate(on_activate);
    app.run();
}

fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    // We have to go through the trait directly as a similar method exists in
    // both GtkRootExt and GtkWidgetExt trait.
    let display = WidgetExt::display(&window);

    window.connect_realize(glib::clone!(@weak display, @weak application => move |_| {
        let provider = ContentProvider::new();
        display.clipboard().set_content(Some(&provider)).unwrap();
        glib::MainContext::default().spawn_local(glib::clone!(@weak display, @weak application => async move {
            let text = display.clipboard().read_text_future().await.unwrap().unwrap();
            assert_eq!(text.as_str(), "Hello clipboard!");
            application.quit();
        }));
    }));

    window.present();
}
