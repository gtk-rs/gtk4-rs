mod imp;

use crate::TOTAL_COLORS;
use gtk::glib;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct CustomLayout(ObjectSubclass<imp::CustomLayout>)
        @extends gtk::LayoutManager;
}

impl Default for CustomLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomLayout {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create CustomLayout")
    }

    pub fn set_position(&self, position: f64) {
        let self_ = imp::CustomLayout::from_instance(self);
        self_.position.set(position);
    }

    pub fn shuffle(&self) {
        let self_ = imp::CustomLayout::from_instance(self);
        for i in 0..TOTAL_COLORS {
            let j = glib::random_int_range(0, i + 1);
            {
                let mut child_pos = self_.child_pos.borrow_mut();
                let tmp = *child_pos.get(i as usize).unwrap();
                let tmp2 = *child_pos.get(j as usize).unwrap();
                child_pos[i as usize] = tmp2;
                child_pos[j as usize] = tmp;
            }
        }
    }
}
