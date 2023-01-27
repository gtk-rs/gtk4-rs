mod video_player_window;

use gtk::glib;
use gtk::prelude::*;
use video_player_window::VideoPlayerWindow;

fn main() -> glib::ExitCode {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.video_player"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let win = VideoPlayerWindow::new(app);
        win.show();
    });

    application.run()
}
