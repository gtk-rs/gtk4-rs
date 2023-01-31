mod imp;

use crate::{Explosion, ExplosionParameters};
use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, glib, graphene};

glib::wrapper! {
    pub struct AnimatedExplosion(ObjectSubclass<imp::AnimatedExplosion>);
}

impl AnimatedExplosion {
    pub(super) fn new(parameters: ExplosionParameters) -> Self {
        let this: Self = glib::Object::new();
        let imp = this.imp();
        imp.explosion.replace(Some(Explosion::new(parameters)));
        this
    }
    pub(super) fn init_time(&self, clock: &gdk::FrameClock, duration: f64) {
        let imp = self.imp();
        let time = clock.frame_time() as f64 / 1000.0;
        imp.start.replace(time);
        imp.lastupdate.replace(time);
        imp.duration.replace(duration);
    }
    // Returns glib::source::Continue(false) if the animation is finished
    pub(super) fn update(&self, clock: &gdk::FrameClock) -> Continue {
        let imp = self.imp();

        // Time tracking to correctly update the physics simulation.
        let time = clock.frame_time() as f64 / 1000.0;
        let dt = { time - imp.lastupdate.get() };
        imp.lastupdate.replace(time);

        imp.explosion
            .borrow_mut()
            .as_mut()
            .unwrap()
            .update(dt as f32);

        if !imp.finished.get() && time - imp.start.get() >= imp.duration.get() {
            imp.finished.replace(true);
        }
        Continue(!imp.finished.get())
    }

    pub(super) fn draw(&self, snapshot: &gtk::Snapshot) {
        let imp = self.imp();

        let e = imp.explosion.borrow();
        let e = e.as_ref().unwrap();
        for i in 0..e.velocity.len() {
            let w = 10.0;
            let h = 10.0;
            let (x, y) = (e.position[i].x(), e.position[i].y());
            snapshot.append_color(
                &e.color[i],
                // Coordinate (0,0) == top-left of the rectangle. I apply a translation to put
                // (0,0) on the center of the rectangle.
                &graphene::Rect::new(x - w / 2.0, y - h / 2.0, w, h),
            );
        }
    }
}
