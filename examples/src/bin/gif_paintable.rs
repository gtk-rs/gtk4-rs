// A custom GDK paintable capable of rendering a GIF
// The paintable makes uses of the awesome image
// crate to read a gif file and transform it to a Vec<Frame>
// Which are then rendered by the paintable at snapshot
// A tick callback that calls invalidate_contents on the paintable
// to render a new frame
use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib, graphene};
use image::{gif::GifDecoder, AnimationDecoder};
use std::env::args;
use std::io::Cursor;

pub struct Frame {
    pub width: i32,
    pub height: i32,
    pub texture: gdk::Paintable,
}

impl Frame {
    pub fn new(f: image::Frame) -> Self {
        let image = f.into_buffer();

        let samples = image.into_flat_samples();
        let (stride, width, height) = samples.extents();
        let width = width as i32;
        let height = height as i32;

        let bytes = glib::Bytes::from(samples.as_slice());

        let texture =
            gdk::MemoryTexture::new(width, height, gdk::MemoryFormat::R8g8b8a8, &bytes, stride);
        Frame {
            width,
            height,
            texture: texture.upcast(),
        }
    }
}

mod imp {
    use super::*;
    use glib::subclass;
    use std::cell::{Cell, RefCell};

    pub struct GifPaintable {
        pub frames: RefCell<Vec<Frame>>,
        pub next_frame: RefCell<Option<gdk::Paintable>>,
        pub width: Cell<i32>,
        pub height: Cell<i32>,
        pub current_idx: Cell<usize>,
    }

    impl ObjectSubclass for GifPaintable {
        const NAME: &'static str = "GifPaintable";
        type Type = super::GifPaintable;
        type ParentType = glib::Object;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib::object_subclass!();

        fn type_init(type_: &mut subclass::InitializingType<Self>) {
            type_.add_interface::<gdk::Paintable>();
        }

        fn new() -> Self {
            Self {
                next_frame: RefCell::new(None),
                frames: RefCell::new(vec![]),
                width: Cell::new(0),
                height: Cell::new(0),
                current_idx: Cell::new(0),
            }
        }
    }

    impl ObjectImpl for GifPaintable {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }

        fn dispose(&self, _obj: &Self::Type) {}
    }

    impl PaintableImpl for GifPaintable {
        fn get_intrinsic_height(&self, _paintable: &Self::Type) -> i32 {
            self.height.get()
        }

        fn get_intrinsic_width(&self, _paintable: &Self::Type) -> i32 {
            self.width.get()
        }

        fn snapshot(
            &self,
            _paintable: &Self::Type,
            snapshot: &gdk::Snapshot,
            width: f64,
            height: f64,
        ) {
            if let Some(texture) = self.next_frame.borrow_mut().take() {
                let w = self.width.get() as f64;
                let h = self.height.get() as f64;
                texture.snapshot(snapshot, w, h);
            } else {
                let snapshot = snapshot.downcast_ref::<gtk::Snapshot>().unwrap();
                snapshot.append_color(
                    &gdk::RGBA::black(),
                    &graphene::Rect::new(0f32, 0f32, width as f32, height as f32),
                );
            }
        }
    }
}

glib::wrapper! {
    pub struct GifPaintable(ObjectSubclass<imp::GifPaintable>) @implements gdk::Paintable;
}

impl GifPaintable {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create object of type GifPaintable")
    }

    // Loads the bytes of a GIF into the paintable
    // The loading consists of decoding the gif with a GIFDecoder
    // Then storing the frames so that the paintable can render them
    pub fn load_form_bytes(&self, bytes: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let self_ = imp::GifPaintable::from_instance(self);
        self_.current_idx.set(0);

        let read = Cursor::new(bytes);
        let decoder = GifDecoder::new(read)?;
        let frames = decoder
            .into_frames()
            .collect_frames()?
            .into_iter()
            .map(Frame::new)
            .collect::<Vec<Frame>>();

        self_.frames.replace(frames);
        self.invalidate_contents();
        glib::timeout_add_local(
            std::time::Duration::from_millis(60),
            glib::clone!(@weak self as paintable => move || {
                paintable.setup_next_frame();
                glib::Continue(true)
            }),
        );
        Ok(())
    }

    fn setup_next_frame(&self) {
        let self_ = imp::GifPaintable::from_instance(self);
        let mut idx = self_.current_idx.get() + 1;
        let frames = self_.frames.borrow();
        if idx >= frames.len() {
            idx = 0;
        }
        self_.current_idx.set(idx);

        let frame = frames.get(idx).unwrap();

        self_.width.set(frame.width);
        self_.height.set(frame.height);
        self_.next_frame.replace(Some(frame.texture.clone()));

        self.invalidate_contents();
    }
}

mod win {
    use super::*;
    use glib::subclass;
    use std::cell::RefCell;

    #[derive(Debug)]
    pub struct Window {
        pub paintable: GifPaintable,
        pub file_chooser: RefCell<Option<gtk::FileChooserNative>>,
    }

    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                paintable: GifPaintable::new(),
                file_chooser: RefCell::new(None),
            }
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self, obj: &Self::Type) {
            obj.setup_widgets();
            obj.setup_actions();
            self.parent_constructed(obj);
        }
    }
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<win::Window>) @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionGroup, gio::ActionMap;
}

impl Window {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create Window")
    }

    fn open_file(&self, file: gio::File) -> Result<(), Box<dyn std::error::Error>> {
        let self_ = win::Window::from_instance(self);
        let (bytes, _) = file.load_contents(gio::NONE_CANCELLABLE)?;
        self_.paintable.load_form_bytes(bytes)?;
        Ok(())
    }

    fn open_file_dialog(&self) {
        let self_ = win::Window::from_instance(self);
        let gif_filter = gtk::FileFilter::new();
        gif_filter.add_mime_type("image/gif");
        gif_filter.set_name(Some("GIF (image/gif)"));

        let file_chooser = gtk::FileChooserNativeBuilder::new()
            .accept_label("Open")
            .cancel_label("Cancel")
            .modal(true)
            .title("Open a GIF to display")
            .transient_for(self)
            .action(gtk::FileChooserAction::Open)
            .build();

        file_chooser.add_filter(&gif_filter);
        file_chooser.connect_response(clone!(@weak self as win => move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                let selected_file = dialog.get_file().unwrap();
                win.open_file(selected_file).unwrap();
            }
            dialog.destroy();
        }));
        file_chooser.show();
        // Hold a reference to the file chooser as it's a native one
        self_.file_chooser.replace(Some(file_chooser));
    }

    fn setup_actions(&self) {
        let open_action = gio::SimpleAction::new("open", None);
        open_action.connect_activate(clone!(@weak self as win => move |_action, _target| {
            win.open_file_dialog();
        }));
        self.add_action(&open_action);
    }

    fn setup_widgets(&self) {
        let self_ = win::Window::from_instance(self);
        // gtk::ApplicationWindow implements gio::ActionMap & gio::ActionGroup interfaces
        // and has an action group called win where we will insert an open action
        // otherwise you can create an action group with gio::SimpleActionGroup
        let open_btn = gtk::ButtonBuilder::new()
            .label("Open")
            .action_name("win.open")
            .build();
        let headerbar = gtk::HeaderBar::new();
        headerbar.pack_start(&open_btn);

        self.set_titlebar(Some(&headerbar));
        self.set_default_size(720, 600);

        let box_ = gtk::Box::new(gtk::Orientation::Vertical, 24);

        let image = gtk::Picture::new();
        image.set_halign(gtk::Align::Center);
        image.set_valign(gtk::Align::Center);
        image.set_size_request(600, 480);
        image.set_paintable(Some(&self_.paintable));
        box_.append(&image);

        self.set_child(Some(&box_));
    }
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.gif-paintable"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        let win = Window::new(app);
        win.show();
    });

    application.run(&args().collect::<Vec<_>>());
}
