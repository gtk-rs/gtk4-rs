mod imp;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct SqueezerBin(ObjectSubclass<imp::SqueezerBin>) @extends gtk::Widget;
}

impl SqueezerBin {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }

    fn get_child_size(&self) -> ((i32, i32), (i32, i32)) {
        let (horizontal_minimal, horizontal_natural, _, _) = self
            .first_child()
            .unwrap()
            .measure(gtk::Orientation::Horizontal, -1);
        let (vertical_minimal, vertical_natural, _, _) = self
            .first_child()
            .unwrap()
            .measure(gtk::Orientation::Vertical, -1);

        (
            (horizontal_minimal, horizontal_natural),
            (vertical_minimal, vertical_natural),
        )
    }

    pub fn imp(&self) -> &imp::SqueezerBin {
        imp::SqueezerBin::from_instance(self)
    }
}
