use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib, graphene};

use std::str::FromStr;

#[derive(Default)]
pub struct SquaresWidget {}

#[glib::object_subclass]
impl ObjectSubclass for SquaresWidget {
    const NAME: &'static str = "SquaresWidget";
    type Type = super::SquaresWidget;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for SquaresWidget {}

impl WidgetImpl for SquaresWidget {
    fn measure(&self, _orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
        // We need some space to draw
        (100, 200, -1, -1)
    }

    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        let widget = self.instance();
        // Draw four color squares
        let width = (widget.width() / 2) as f32;
        let height = (widget.height() / 2) as f32;

        let red_color = gdk::RGBA::RED;
        let rect = graphene::Rect::new(0_f32, 0_f32, width, height);
        snapshot.append_color(&red_color, &rect);

        let green_color = gdk::RGBA::GREEN;
        let rect = graphene::Rect::new(width, 0_f32, width, height);
        snapshot.append_color(&green_color, &rect);

        let yellow_color = gdk::RGBA::from_str("yellow").expect("Failed to parse string");
        let rect = graphene::Rect::new(0_f32, height, width, height);
        snapshot.append_color(&yellow_color, &rect);

        let blue_color = gdk::RGBA::BLUE;
        let rect = graphene::Rect::new(width, height, width, height);
        snapshot.append_color(&blue_color, &rect);
    }
}
