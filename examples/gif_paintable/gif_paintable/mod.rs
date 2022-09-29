mod frame;
mod imp;

use std::io::Cursor;

use frame::Frame;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib};
use image::{codecs::gif::GifDecoder, AnimationDecoder};

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
        let imp = self.imp();
        imp.current_idx.set(0);

        let read = Cursor::new(bytes);
        let decoder = GifDecoder::new(read)?;
        let frames = decoder
            .into_frames()
            .collect_frames()?
            .into_iter()
            .map(Frame::new)
            .collect::<Vec<Frame>>();

        imp.frames.replace(Some(frames));

        // make sure the first frame is queued to play
        self.setup_next_frame();

        Ok(())
    }

    fn setup_next_frame(&self) {
        let imp = self.imp();
        let idx = imp.current_idx.get();
        let frames_ref = imp.frames.borrow();

        // if we have stored no frames then we early return early
        // and instead render a default frame in `imp::GifPaintable::snapshot`
        let frames = match &*frames_ref {
            Some(frames) => frames,
            None => return,
        };

        let next_frame = frames.get(idx).unwrap();

        imp.width.set(Some(next_frame.width));
        imp.height.set(Some(next_frame.height));
        imp.next_frame.replace(Some(next_frame.texture.clone()));

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
        imp.current_idx.set(new_idx);
    }
}

impl Default for GifPaintable {
    fn default() -> Self {
        Self::new()
    }
}
