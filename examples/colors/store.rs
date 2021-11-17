use crate::color::Color;
use gtk::{gdk, glib, prelude::*, subclass::prelude::*};

const N_COLORS: usize = 256 * 256 * 256;

mod imp {
    use super::*;
    use glib::{ParamFlags, ParamSpec, ParamSpecUInt, Value};
    use once_cell::sync::Lazy;
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::cmp::Ordering;

    pub struct ColorStore {
        pub colors: RefCell<Vec<Color>>,
        pub size: Cell<u32>,
        pub limit: Cell<u32>,
    }

    impl Default for ColorStore {
        fn default() -> Self {
            Self {
                colors: RefCell::new(vec![Color::new(gdk::RGBA::RED); N_COLORS]),
                size: Cell::new(0),
                limit: Cell::new(0),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColorStore {
        const NAME: &'static str = "ColorStore";
        type Type = super::ColorStore;
        type Interfaces = (gtk::gio::ListModel,);
    }

    impl ObjectImpl for ColorStore {
        fn constructed(&self, obj: &Self::Type) {
            self.generate(obj);
            self.parent_constructed(obj);
        }

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecUInt::new(
                        "size",
                        "size",
                        "size",
                        u32::MIN,
                        u32::MAX,
                        0,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecUInt::new(
                        "limit",
                        "limit",
                        "limit",
                        u32::MIN,
                        u32::MAX,
                        0,
                        ParamFlags::READWRITE,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "size" => self.set_size(_obj, value.get().unwrap()),
                "limit" => {
                    self.limit.replace(value.get().unwrap());
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "size" => (self.size.get()).to_value(),
                "limit" => (self.limit.get()).to_value(),
                _ => unimplemented!(),
            }
        }
    }

    impl ColorStore {
        pub fn generate(&self, obj: &super::ColorStore) {
            let iter = include_str!("color.names.txt").split('\n');

            let mut size = 0;
            for line in iter {
                if !line.starts_with('#')
                    && !line.starts_with('\0')
                    && !line.is_empty()
                    && size < N_COLORS
                {
                    let mut fields = line.split(' ');
                    if let (_, Some(_name), _, Some(red), Some(green), Some(blue)) = (
                        fields.next(),
                        fields.next(),
                        fields.next(),
                        fields.next().and_then(|f| str::parse::<f32>(f).ok()),
                        fields.next().and_then(|f| str::parse::<f32>(f).ok()),
                        fields.next().and_then(|f| str::parse::<f32>(f).ok()),
                    ) {
                        let rgba = gdk::RGBA::new(red / 255.0, green / 255.0, blue / 255.0, 1.0);

                        let pos = ((red as usize & 0xFF) << 16)
                            | ((green as usize & 0xFF) << 8)
                            | blue as usize;

                        if let Some(c) = self.colors.borrow_mut().get_mut(pos as usize) {
                            c.set_property("color", rgba)
                        }

                        obj.items_changed(pos as u32, 0, 1);

                        size += 1;
                    }
                }
            }
        }

        pub fn set_size(&self, obj: &super::ColorStore, size: u32) {
            let old_size = self.size.get();

            self.size.replace(size);

            match size.cmp(&old_size) {
                Ordering::Less => obj.items_changed(size, old_size - size, 0),
                Ordering::Greater => obj.items_changed(old_size, 0, size - old_size),
                Ordering::Equal => (),
            }

            obj.notify_by_pspec(&obj.list_properties()[0]);
        }
    }

    impl ListModelImpl for ColorStore {
        fn item_type(&self, _: &Self::Type) -> glib::Type {
            Color::static_type()
        }

        fn n_items(&self, _: &Self::Type) -> u32 {
            self.size.get()
        }

        fn item(&self, _: &Self::Type, position: u32) -> Option<glib::Object> {
            let map = [
                0xFF0000, 0x00FF00, 0x0000FF, 0x7F0000, 0x007F00, 0x00007F, 0x3F0000, 0x003F00,
                0x00003F, 0x1F0000, 0x001F00, 0x00001F, 0x0F0000, 0x000F00, 0x00000F, 0x070000,
                0x000700, 0x000007, 0x030000, 0x000300, 0x000003, 0x010000, 0x000100, 0x000001,
            ];

            let mut result = 0;

            for (i, item) in map.iter().enumerate() {
                if position & (1 << i) != 0 {
                    result ^= item;
                }
            }

            let color = Color::new(gdk::RGBA::new(
                ((result >> 16) & 0xFF) as f32 / 255.,
                ((result >> 8) & 0xFF) as f32 / 255.,
                (result & 0xFF) as f32 / 255.,
                1.0,
            ));

            Some(color.upcast::<glib::Object>())
        }
    }
}

glib::wrapper! {
    pub struct ColorStore(ObjectSubclass<imp::ColorStore>) @implements gtk::gio::ListModel;
}

impl ColorStore {
    pub fn new(size: u32) -> Self {
        glib::Object::new(&[("size", &size.to_value())]).expect("Failed to create `ColorStore`.")
    }

    pub fn size(&self) -> u32 {
        self.property("size")
    }

    pub fn limit(&self) -> u32 {
        self.property("limit")
    }
}
