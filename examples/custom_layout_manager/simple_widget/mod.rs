mod imp;

use crate::custom_layout::CustomLayout;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct SimpleWidget(ObjectSubclass<imp::SimpleWidget>)
        @extends gtk::Widget;
}

impl Default for SimpleWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleWidget {
    pub fn new() -> Self {
        glib::Object::new_default()
    }

    pub fn add_child<W: IsA<gtk::Widget>>(&self, widget: &W) {
        widget.set_parent(self);
    }

    pub fn do_transition(&self) {
        let imp = self.imp();
        if imp.tick_id.borrow().is_some() {
            return;
        }

        let start_time = std::time::Instant::now();
        imp.start_time.replace(Some(start_time));
        let tick_id =
            self.add_tick_callback(clone!(@weak self as this => @default-panic, move |_, _| {
                this.transition()
            }));
        imp.tick_id.replace(Some(tick_id));
    }

    pub fn transition(&self) -> glib::Continue {
        let imp = self.imp();
        let now = std::time::Instant::now();
        self.queue_allocate();

        let start_time = imp.start_time.borrow().unwrap();

        let layout_manager = self
            .layout_manager()
            .and_downcast::<CustomLayout>()
            .unwrap();

        let duration_diff = now.duration_since(start_time);
        let diff_secs =
            duration_diff.as_secs_f64() / std::time::Duration::from_secs_f64(0.5).as_secs_f64();
        if imp.backward.get() {
            layout_manager.set_position(1.0 - diff_secs);
        } else {
            layout_manager.set_position(diff_secs);
        }

        if diff_secs > 1.0 {
            let is_backward = !imp.backward.get();
            imp.backward.set(is_backward);

            if is_backward {
                layout_manager.set_position(1.0);
            } else {
                layout_manager.shuffle();
                layout_manager.set_position(0.0);
            }
            let _ = imp.tick_id.borrow_mut().take();
            return glib::Continue(false);
        }

        glib::Continue(true)
    }
}
