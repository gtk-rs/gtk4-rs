// A custom GDK paintable capable of rendering a GIF
// The paintable makes uses of the awesome image
// crate to read a gif file and transform it to a Vec<Frame>
// which are then rendered by the paintable at different snapshots

mod gif_paintable;
mod gif_paintable_window;

use gif_paintable_window::GifPaintableWindow;
use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.gif_paintable"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let win = GifPaintableWindow::new(app);
        win.show();
    });

    application.run();
}
