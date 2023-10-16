mod imp;

use gtk::{
    glib::{self, clone, ControlFlow},
    prelude::*,
    subclass::prelude::*,
};

use crate::{AnimatedExplosion, ExplosionParameters};

glib::wrapper! {
    pub struct ConfettiWidget(ObjectSubclass<imp::ConfettiWidget>) @implements gtk::Widget;
}

impl Default for ConfettiWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfettiWidget {
    pub fn new() -> Self {
        glib::Object::new()
    }
    pub fn explode(&self, params: ExplosionParameters, duration: f64) -> AnimatedExplosion {
        let exp = AnimatedExplosion::new(params);

        // A FrameClock tells the application when to update and repaint a surface.
        // This may be synced to the vertical refresh rate of the monitor, for example.
        let frame_clock = self.frame_clock().unwrap();
        exp.init_time(&frame_clock, duration);

        frame_clock.connect_update(clone!(@weak self as this, @weak exp => move |clock| {
            match exp.update(clock) {
                ControlFlow::Continue => {
                    this.queue_draw();
                },
                ControlFlow::Break => {
                    this.imp().explosions.borrow_mut().remove(&exp);
                    clock.end_updating();
                }
            }
        }));
        self.imp().explosions.borrow_mut().insert(exp.clone());
        frame_clock.begin_updating();
        exp
    }
}
