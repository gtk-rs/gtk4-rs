use graphene::Point;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, graphene, gsk, SizeRequestMode};

use super::Rotation;
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

#[derive(Debug, Default)]
pub struct RotationBin {
    child: RefCell<Option<gtk::Widget>>,
    rotation: Cell<Rotation>,
}

#[glib::object_subclass]
impl ObjectSubclass for RotationBin {
    const NAME: &'static str = "RotationBin";
    type Type = super::RotationBin;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for RotationBin {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecObject::builder::<gtk::Widget>("child")
                    .explicit_notify()
                    .build(),
                glib::ParamSpecEnum::builder::<Rotation>("rotation")
                    .explicit_notify()
                    .build(),
            ]
        });

        PROPERTIES.as_ref()
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        let obj = self.obj();

        match pspec.name() {
            "child" => self.child(&obj).to_value(),
            "rotation" => self.rotation(&obj).to_value(),
            _ => unimplemented!(),
        }
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        let obj = self.obj();

        match pspec.name() {
            "child" => {
                self.set_child(&obj, value.get::<gtk::Widget>().ok().as_ref());
            }
            "rotation" => {
                self.set_rotation(&obj, value.get().unwrap());
            }
            _ => unimplemented!(),
        }
    }

    fn dispose(&self) {
        if let Some(child) = self.child.borrow_mut().take() {
            child.unparent();
        }
    }
}

impl WidgetImpl for RotationBin {
    // When rotated by 90 or 270 degrees swap HeightForWidth with WidthForHeight
    fn request_mode(&self) -> SizeRequestMode {
        let widget = self.obj();
        let child = self.child.borrow();
        let child = match child.as_ref() {
            Some(child) => child,
            None => return SizeRequestMode::ConstantSize,
        };

        let mode = child.request_mode();

        match self.rotation(&widget) {
            Rotation::Normal | Rotation::Deg180 => return mode,
            _ => (),
        }

        match mode {
            SizeRequestMode::ConstantSize => SizeRequestMode::ConstantSize,
            SizeRequestMode::HeightForWidth => SizeRequestMode::WidthForHeight,
            SizeRequestMode::WidthForHeight => SizeRequestMode::HeightForWidth,
            _ => unreachable!(),
        }
    }

    // When rotated by 90 or 270 degrees, swap the orientation which expand
    // properties are computed against.
    fn compute_expand(&self, hexpand: &mut bool, vexpand: &mut bool) {
        let child = self.child.borrow();
        let child = match child.as_ref() {
            Some(child) => child,
            None => {
                *hexpand = false;
                *vexpand = false;
                return;
            }
        };

        match self.rotation(&self.obj()) {
            Rotation::Normal | Rotation::Deg180 => {
                *hexpand = child.compute_expand(gtk::Orientation::Horizontal);
                *vexpand = child.compute_expand(gtk::Orientation::Vertical);
            }
            Rotation::Deg90 | Rotation::Deg270 => {
                *hexpand = child.compute_expand(gtk::Orientation::Vertical);
                *vexpand = child.compute_expand(gtk::Orientation::Horizontal);
            }
        };
    }

    fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        let widget = self.obj();
        let child = self.child.borrow();
        let child = match child.as_ref() {
            Some(child) => child,
            None => return (0, 0, -1, -1),
        };

        // Swap the Orientation for 90 or 270 degrees
        let orientation = match self.rotation(&widget) {
            Rotation::Deg90 | Rotation::Deg270 => {
                if orientation == gtk::Orientation::Horizontal {
                    gtk::Orientation::Vertical
                } else {
                    gtk::Orientation::Horizontal
                }
            }
            _ => orientation,
        };

        // If the orientation is horizontal and the rotation 0 or 180 degrees,
        // then pass through the child's measure directly
        //
        // Otherwise return child's measure but discard minimum_baseline and natural_baseline
        // as baseline is only useful for vertical orientations
        match (orientation, self.rotation(&widget)) {
            (gtk::Orientation::Horizontal, Rotation::Normal | Rotation::Deg180) => {
                child.measure(orientation, for_size)
            }
            (_, _) => {
                let (min, nat, _, _) = child.measure(orientation, for_size);
                (min, nat, -1, -1)
            }
        }
    }

    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        let widget = self.obj();
        let child = self.child.borrow();
        let child = match child.as_ref() {
            Some(child) => child,
            None => return,
        };

        // Swap height and width for 90 or 270 degrees
        let (w, h) = match self.rotation(&widget) {
            Rotation::Deg90 | Rotation::Deg270 => (height, width),
            _ => (width, height),
        };

        let transform = gsk::Transform::new();
        // Transformation are done with respect to their origin point, top-left
        // by default in GTK. Move the origin of the transformation to the
        // center, instead of a corner, by taking half of width and height.
        let transform = transform
            .translate(&Point::new(width as f32 / 2.0, height as f32 / 2.0))
            // Rotate by the desired angle
            .rotate(self.rotation(&widget).into())
            // Revert the move of the origin point once our rotation is done.
            .translate(&Point::new(-w as f32 / 2.0, -h as f32 / 2.0));
        child.allocate(w, h, baseline, Some(transform))
    }
}

impl RotationBin {
    pub(super) fn child(&self, _obj: &super::RotationBin) -> Option<gtk::Widget> {
        self.child.borrow().clone()
    }

    pub(super) fn rotation(&self, _obj: &super::RotationBin) -> Rotation {
        self.rotation.get()
    }

    pub(super) fn set_child(
        &self,
        obj: &super::RotationBin,
        widget: Option<&impl IsA<gtk::Widget>>,
    ) {
        let widget = widget.map(|w| w.upcast_ref());
        if widget == self.child.borrow().as_ref() {
            return;
        }

        if let Some(child) = self.child.borrow_mut().take() {
            child.unparent();
        }

        if let Some(w) = widget {
            self.child.replace(Some(w.clone()));
            w.set_parent(obj);
        }

        obj.queue_resize();
        obj.notify("child")
    }

    pub(super) fn set_rotation(&self, obj: &super::RotationBin, rotation: Rotation) {
        if self.rotation.get() != rotation {
            self.rotation.replace(rotation);
            obj.queue_resize();
            obj.notify("rotation");
        }
    }

    pub(super) fn rotate_clockwise(&self, obj: &super::RotationBin) {
        let r = self.rotation(obj);
        self.set_rotation(obj, r.rotate_clockwise());
    }

    pub(super) fn rotate_counter_clockwise(&self, obj: &super::RotationBin) {
        let r = self.rotation(obj);
        self.set_rotation(obj, r.rotate_counter_clockwise());
    }
}
