mod imp;

use gtk::{glib, subclass::prelude::*};

use crate::TOTAL_COLORS;

glib::wrapper! {
    pub struct CustomLayout(ObjectSubclass<imp::CustomLayout>)
        @extends gtk::LayoutManager;
}

impl CustomLayout {
    pub fn set_position(&self, position: f64) {
        self.imp().position.set(position);
    }

    pub fn shuffle(&self) {
        let imp = self.imp();
        for i in 0..TOTAL_COLORS {
            let j = glib::random_int_range(0, i + 1);
            {
                let mut child_pos = imp.child_pos.borrow_mut();
                let tmp = *child_pos.get(i as usize).unwrap();
                let tmp2 = *child_pos.get(j as usize).unwrap();
                child_pos[i as usize] = tmp2;
                child_pos[j as usize] = tmp;
            }
        }
    }
}
