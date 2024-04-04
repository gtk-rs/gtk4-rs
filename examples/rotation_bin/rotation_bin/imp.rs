use std::cell::{Cell, RefCell};

use graphene::Point;
use gtk::{glib, graphene, gsk, prelude::*, subclass::prelude::*, SizeRequestMode};

use super::Rotation;

#[derive(Debug, glib::Properties, Default)]
#[properties(wrapper_type = super::RotationBin)]
pub struct RotationBin {
    #[property(get, explicit_notify)]
    pub(super) child: RefCell<Option<gtk::Widget>>,
    #[property(get, explicit_notify, builder(Rotation::Normal))]
    pub(super) rotation: Cell<Rotation>,
}

#[glib::object_subclass]
impl ObjectSubclass for RotationBin {
    const NAME: &'static str = "RotationBin";
    type Type = super::RotationBin;
    type ParentType = gtk::Widget;
}

#[glib::derived_properties]
impl ObjectImpl for RotationBin {
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

        match widget.rotation() {
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

        match self.obj().rotation() {
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
        let orientation = match widget.rotation() {
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
        // Otherwise return child's measure but discard minimum_baseline and
        // natural_baseline as baseline is only useful for vertical orientations
        match (orientation, widget.rotation()) {
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
        let (w, h) = match widget.rotation() {
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
            .rotate(widget.rotation().into())
            // Revert the move of the origin point once our rotation is done.
            .translate(&Point::new(-w as f32 / 2.0, -h as f32 / 2.0));
        child.allocate(w, h, baseline, Some(transform))
    }
}
