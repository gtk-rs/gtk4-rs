mod imp;

use gtk::{
    glib::{self, clone, ControlFlow},
    prelude::*,
    subclass::prelude::*,
};

use crate::{AnimatedExplosion, ExplosionParameters};

glib::wrapper! {
    pub struct ConfettiWidget(ObjectSubclass<imp::ConfettiWidget>)
    @extends gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ConfettiWidget {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ConfettiWidget {
    pub fn explode(&self, params: ExplosionParameters, duration: f64) -> AnimatedExplosion {
        let exp = AnimatedExplosion::new(params);

        // A FrameClock tells the application when to update and repaint a surface.
        // This may be synced to the vertical refresh rate of the monitor, for example.
        let frame_clock = self.frame_clock().unwrap();
        exp.init_time(&frame_clock, duration);

        frame_clock.connect_update(clone!(
            #[weak(rename_to = this)]
            self,
            #[weak]
            exp,
            move |clock| {
                match exp.update(clock) {
                    ControlFlow::Continue => {
                        this.queue_draw();
                    }
                    ControlFlow::Break => {
                        this.imp().explosions.borrow_mut().remove(&exp);
                        clock.end_updating();
                    }
                }
            }
        ));
        self.imp().explosions.borrow_mut().insert(exp.clone());
        frame_clock.begin_updating();
        exp
    }
}
