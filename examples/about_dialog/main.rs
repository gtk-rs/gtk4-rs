use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;

static LOGO_SVG: &[u8] = include_bytes!("gtk-rs.svg");

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();

    app.connect_activate(|app| {
        let button = gtk::Button::builder()
            .label("Show About Dialog")
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(24)
            .margin_end(24)
            .build();

        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Example")
            .child(&button)
            .build();

        let bytes = glib::Bytes::from_static(LOGO_SVG);
        let logo = gdk::Texture::from_bytes(&bytes).expect("gtk-rs.svg to load");

        button.connect_clicked(glib::clone!(
            #[weak]
            window,
            move |_| {
                let dialog = gtk::AboutDialog::builder()
                    .transient_for(&window)
                    .modal(true)
                    .program_name("About Dialog Example")
                    .version("0.1.0")
                    .website("https://gtk-rs.org")
                    .license_type(gtk::License::MitX11)
                    .authors(["Author 1", "Author 2"])
                    .logo(&logo)
                    .build();

                dialog.present();
            }
        ));

        window.present();
    });

    app.run();
}
