use std::{cell::RefCell, collections::HashSet};

use gtk::{glib, subclass::prelude::*};

use crate::AnimatedExplosion;

#[derive(Default)]
pub struct ConfettiWidget {
    pub(crate) explosions: RefCell<HashSet<AnimatedExplosion>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ConfettiWidget {
    const NAME: &'static str = "ConfettiWidget";
    type Type = super::ConfettiWidget;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for ConfettiWidget {}
impl WidgetImpl for ConfettiWidget {
    // We override the snapshot virtual function to draw custom graphics
    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        for e in self.explosions.borrow().iter() {
            e.draw(snapshot);
        }
    }
}
