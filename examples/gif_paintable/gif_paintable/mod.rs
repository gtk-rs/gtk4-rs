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

    /// Loads the bytes of a GIF into the paintable.
    ///
    /// The loading consists of decoding the gif with a GIFDecoder, then storing
    /// the frames so that the paintable can render them.
    pub fn load_from_bytes(&self, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let imp = self.imp();
        imp.current_idx.set(0);

        if let Some(source_id) = imp.timeout_source_id.take() {
            source_id.remove();
        }

        let read = Cursor::new(bytes);

        // Images from unknown origins make a program vulnerable to
        // decompression bombs. That is, malicious images crafted specifically
        // to require an enormous amount of memory to process while having a
        // disproportionately small file size.
        //
        // By default, `GifDecoder::new()` limits the allocation of a single
        // frame to 50MB, but it can be restricted further with
        // `GifDecoder::with_limits()`.
        //
        // An safety measure to guard against that would be to process each
        // frame as needed instead of loading them all with `collect_frames()`.
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
        imp.next_frame.replace(Some(next_frame.texture.clone()));

        // invalidate the contents so that the new frame will be rendered
        self.invalidate_contents();

        // setup a callback to this function once the frame has finished so that
        // we can play the next frame
        let update_next_frame_callback = glib::clone!(@weak self as paintable => move || {
            paintable.imp().timeout_source_id.take();
            paintable.setup_next_frame();
        });

        let source_id =
            glib::timeout_add_local_once(next_frame.frame_duration, update_next_frame_callback);
        imp.timeout_source_id.replace(Some(source_id));

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
