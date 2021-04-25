use crate::custom_layout::CustomLayout;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(Debug)]
    pub struct SimpleWidget {
        pub backward: Cell<bool>,
        pub tick_id: RefCell<Option<gtk::TickCallbackId>>,
        pub start_time: RefCell<Option<std::time::Instant>>,
    }

    impl Default for SimpleWidget {
        fn default() -> Self {
            Self {
                backward: Cell::new(false),
                tick_id: RefCell::default(),
                start_time: RefCell::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SimpleWidget {
        const NAME: &'static str = "SimpleWidget";
        type Type = super::SimpleWidget;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            // We make use of the custom layout manager
            klass.set_layout_manager_type::<CustomLayout>();
        }
    }

    impl ObjectImpl for SimpleWidget {
        fn constructed(&self, obj: &Self::Type) {
            let gesture = gtk::GestureClick::new();
            // Trigger a transition on click
            gesture.connect_pressed(clone!(@strong obj as this => move |_, _, _, _| {
                this.do_transition();
            }));
            obj.add_controller(&gesture);
        }

        fn dispose(&self, widget: &Self::Type) {
            while let Some(child) = widget.first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for SimpleWidget {}
}

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
        glib::Object::new(&[]).expect("Failed to create SimpleWidget")
    }

    pub fn add_child<W: IsA<gtk::Widget>>(&self, widget: &W) {
        widget.set_parent(self);
    }

    pub fn do_transition(&self) {
        let self_ = imp::SimpleWidget::from_instance(self);
        if self_.tick_id.borrow().is_some() {
            return;
        }

        let start_time = std::time::Instant::now();
        self_.start_time.replace(Some(start_time));
        let tick_id =
            self.add_tick_callback(clone!(@weak self as this => @default-panic, move |_, _| {
                this.transition()
            }));
        self_.tick_id.replace(Some(tick_id));
    }

    pub fn transition(&self) -> glib::Continue {
        let self_ = imp::SimpleWidget::from_instance(self);
        let now = std::time::Instant::now();
        self.queue_allocate();

        let start_time = self_.start_time.borrow().unwrap();

        let layout_manager = self
            .layout_manager()
            .unwrap()
            .downcast::<CustomLayout>()
            .unwrap();

        let duration_diff = now.duration_since(start_time);
        let diff_secs =
            duration_diff.as_secs_f64() / std::time::Duration::from_secs_f64(0.5).as_secs_f64();
        if self_.backward.get() {
            layout_manager.set_position(1.0 - diff_secs);
        } else {
            layout_manager.set_position(diff_secs);
        }

        if diff_secs > 1.0 {
            let is_backward = !self_.backward.get();
            self_.backward.set(is_backward);

            if is_backward {
                layout_manager.set_position(1.0);
            } else {
                layout_manager.shuffle();
                layout_manager.set_position(0.0);
            }
            let _ = self_.tick_id.borrow_mut().take();
            return glib::Continue(false);
        }

        glib::Continue(true)
    }
}
