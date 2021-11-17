use gtk::{gdk, glib, prelude::*, subclass::prelude::*};

mod imp {
    use super::*;
    use glib::{ParamFlags, ParamSpec, ParamSpecBoxed, ParamSpecFloat, ParamSpecString, Value};
    use once_cell::sync::Lazy;
    use std::cell::Cell;
    use std::cell::RefCell;

    pub struct Color {
        name: RefCell<String>,
        color: Cell<gdk::RGBA>,
        red: Cell<f32>,
        green: Cell<f32>,
        blue: Cell<f32>,
        h: Cell<f32>,
        s: Cell<f32>,
        v: Cell<f32>,
    }

    impl Default for Color {
        fn default() -> Self {
            Self {
                name: Default::default(),
                color: Cell::new(gdk::RGBA::BLACK),
                red: Cell::new(0.),
                green: Cell::new(0.),
                blue: Cell::new(0.),
                h: Cell::new(0.),
                s: Cell::new(0.),
                v: Cell::new(0.),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Color {
        const NAME: &'static str = "Color";
        type Type = super::Color;
        type Interfaces = (gdk::Paintable,);
    }

    impl ObjectImpl for Color {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new("name", "name", "name", None, ParamFlags::READWRITE),
                    ParamSpecBoxed::new(
                        "color",
                        "color",
                        "color",
                        gdk::RGBA::static_type(),
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecFloat::new(
                        "red",
                        "red",
                        "red",
                        f32::MIN,
                        f32::MAX,
                        0.,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecFloat::new(
                        "green",
                        "green",
                        "green",
                        f32::MIN,
                        f32::MAX,
                        0.,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecFloat::new(
                        "blue",
                        "blue",
                        "blue",
                        f32::MIN,
                        f32::MAX,
                        0.,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecFloat::new(
                        "hue",
                        "hue",
                        "hue",
                        f32::MIN,
                        f32::MAX,
                        0.,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecFloat::new(
                        "saturation",
                        "saturation",
                        "saturation",
                        f32::MIN,
                        f32::MAX,
                        0.,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecFloat::new(
                        "value",
                        "value",
                        "value",
                        f32::MIN,
                        f32::MAX,
                        0.,
                        ParamFlags::READWRITE,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "color" => {
                    let rgba = value.get().expect("The value needs to be of type `i32`.");
                    self.color.replace(rgba);

                    self.red.replace(rgba.red());
                    self.green.replace(rgba.green());
                    self.blue.replace(rgba.blue());

                    let (h, s, v) = gtk::rgb_to_hsv(rgba.red(), rgba.green(), rgba.blue());
                    self.h.replace(h);
                    self.s.replace(s);
                    self.v.replace(v);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "name" => self.name.borrow().to_value(),
                "color" => self.color.get().to_value(),
                "red" => self.red.get().to_value(),
                "green" => self.green.get().to_value(),
                "blue" => self.blue.get().to_value(),
                "hue" => self.h.get().to_value(),
                "saturation" => self.s.get().to_value(),
                "value" => self.v.get().to_value(),
                _ => unimplemented!(),
            }
        }
    }

    impl PaintableImpl for Color {
        fn flags(&self, _paintable: &Self::Type) -> gdk::PaintableFlags {
            gdk::PaintableFlags::SIZE
        }

        fn intrinsic_width(&self, _paintable: &Self::Type) -> i32 {
            32
        }

        fn intrinsic_height(&self, _paintable: &Self::Type) -> i32 {
            32
        }

        fn snapshot(
            &self,
            paintable: &Self::Type,
            snapshot: &gdk::Snapshot,
            width: f64,
            height: f64,
        ) {
            let snapshot = snapshot.downcast_ref::<gtk::Snapshot>().unwrap();
            let color = paintable.color();
            snapshot.append_color(
                &color,
                &gtk::graphene::Rect::new(0_f32, 0_f32, width as f32, height as f32),
            );
        }
    }
}

glib::wrapper! {
    pub struct Color(ObjectSubclass<imp::Color>) @implements gdk::Paintable;
}

impl Color {
    pub fn new(color: gdk::RGBA) -> Self {
        glib::Object::new(&[("color", &color)]).expect("Failed to create `Color`.")
    }

    pub fn color(&self) -> gdk::RGBA {
        self.property::<gdk::RGBA>("color")
    }

    pub fn red(&self) -> f32 {
        self.property::<f32>("red")
    }

    pub fn green(&self) -> f32 {
        self.property::<f32>("green")
    }

    pub fn blue(&self) -> f32 {
        self.property::<f32>("blue")
    }
}
