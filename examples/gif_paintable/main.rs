// A custom GDK paintable capable of rendering a GIF
// The paintable makes uses of the awesome image
// crate to read a gif file and transform it to a Vec<Frame>
// which are then rendered by the paintable at different snapshots

use gtk::{gdk, glib, graphene};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Picture;

use image::{gif::GifDecoder, AnimationDecoder};

use std::io::{Cursor, Read};
use std::time::Duration;
use std::ops::Deref;

pub struct Frame {
    pub width: i32,
    pub height: i32,
    pub texture: gdk::Paintable,
    pub frame_duration: Duration,
}

impl Frame {
    pub fn new(f: image::Frame) -> Self {
        // transform the fractional time into a `Duration`
        let (numerator, denominator) = f.delay().numer_denom_ms();
        let frame_duration = Duration::from_millis((numerator as f64 / denominator as f64) as u64);

        let image = f.into_buffer();

        let samples = image.into_flat_samples();
        let (stride, width, height) = samples.extents();
        
        // glib::Bytes must contain gtk_stride * height number of bytes.
        // since each pixel in the gif frame contains `stride` bytes 
        // of information we must map this stride to a value that gdk 
        // understands (see https://gtk-rs.org/gtk4-rs/git/docs/gdk4/struct.MemoryTexture.html)
        let gtk_stride = stride * width;

        let width = width as i32;
        let height = height as i32;

        let bytes = glib::Bytes::from(samples.as_slice());

        let texture = gdk::MemoryTexture::new(
            width,
            height,
            gdk::MemoryFormat::R8g8b8a8,
            &bytes,
            gtk_stride,
        );

        Frame {
            width,
            height,
            texture: texture.upcast(),
            frame_duration,
        }
    }
}

mod imp {
    use super::*;
    use glib::subclass;
    use std::cell::{Cell, RefCell};

    pub struct GifPaintable {
        pub frames: RefCell<Option<Vec<Frame>>>,
        pub next_frame: RefCell<Option<gdk::Paintable>>,
        pub width: Cell<Option<i32>>,
        pub height: Cell<Option<i32>>,
        pub current_idx: Cell<usize>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GifPaintable {
        const NAME: &'static str = "GifPaintable";
        type Type = super::GifPaintable;
        type ParentType = glib::Object;
        type Instance = subclass::basic::InstanceStruct<Self>;
        type Class = subclass::basic::ClassStruct<Self>;
        type Interfaces = (gdk::Paintable,);

        fn new() -> Self {
            Self {
                next_frame: RefCell::new(None),
                frames: RefCell::new(None),
                width: Cell::new(Some(0)),
                height: Cell::new(Some(0)),
                current_idx: Cell::new(0),
            }
        }
    }

    impl ObjectImpl for GifPaintable {}

    impl PaintableImpl for GifPaintable {
        fn intrinsic_height(&self, _paintable: &Self::Type) -> i32 {
            self.height.get().unwrap_or(-1)
        }

        fn intrinsic_width(&self, _paintable: &Self::Type) -> i32 {
            self.width.get().unwrap_or(-1)
        }

        fn snapshot(
            &self,
            _paintable: &Self::Type,
            snapshot: &gdk::Snapshot,
            width: f64,
            height: f64,
        ) {
            if let (Some(texture), Some(width), Some(height)) = (
                self.next_frame.borrow_mut().take(),
                self.width.take(),
                self.height.take(),
            ) {
                let w = width as f64;
                let h = height as f64;
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
    //
    // You can only call this function once due to the callback system in setup_next_frame
    // causing the frames to refresh at faster intervals
    pub fn load_from_bytes(&self, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
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

        self_.frames.replace(Some(frames));

        // make sure the first frame is queued to play
        self.setup_next_frame();

        Ok(())
    }

    fn setup_next_frame(&self) {
        let self_ = imp::GifPaintable::from_instance(self);
        let idx = self_.current_idx.get();
        let frames_ref = self_.frames.borrow();

        // if we have stored no frames then we early return early
        // and instead render a default frame in `imp::GifPaintable::snapshot`
        let frames = match frames_ref.deref() {
            Some(frames) => frames,
            None => return
        };

        let next_frame = frames.get(idx).unwrap();

        self_.width.set(Some(next_frame.width));
        self_.height.set(Some(next_frame.height));
        self_.next_frame.replace(Some(next_frame.texture.clone()));

        // invalidate the contents so that the new frame will be rendered
        self.invalidate_contents();

        // setup a callback to this function once the frame has finished so that
        // we can play the next frame
        let update_next_frame_callback = glib::clone!(@weak self as paintable => move || {
            paintable.setup_next_frame();
        });

        glib::timeout_add_local_once(next_frame.frame_duration, update_next_frame_callback);

        // setup the index for the next call to setup_next_frame
        let mut new_idx = idx + 1;
        if new_idx >= frames.len() {
            new_idx = 0;
        }
        self_.current_idx.set(new_idx);
    }
}

fn main() {
    let gif = std::env::args().into_iter().skip(1).next().expect("Expected a path to a gif as a command line argument");
    let gif_path = std::path::PathBuf::from(&gif);

    if gif_path.extension().unwrap() != "gif" {
        panic!("Provided argument `{}` did not have a `.gif` file extension.", gif);
    }

    // read in the bytes from the file
    let mut bytes= Vec::new();
    let mut file = std::fs::File::open(&gif_path).expect("could not open gif path");
    file.read_to_end(&mut bytes).expect("Error when reading gif file");

    // setup ui stuff
    let application = Application::new(Some("com.github.gtk-rs.gif-paintable"), Default::default());

    application.connect_activate(move |app| {
        let child = GifPaintable::new();
        child.load_from_bytes(bytes.as_slice()).unwrap();

        let picture = Picture::builder().paintable(&child).build();
        let win = ApplicationWindow::builder()
            .application(app)
            .child(&picture)
            .build();

        win.show();
    });

    application.run();
}
