use std::time::Duration;

use gtk::prelude::*;
use gtk::{gdk, glib};

pub struct Frame {
    pub texture: gdk::Texture,
    pub frame_duration: Duration,
}

impl From<image::Frame> for Frame {
    fn from(f: image::Frame) -> Self {
        let mut frame_duration = Duration::from(f.delay());

        // convention is to use 100 milliseconds duration if it is defined as 0.
        if frame_duration.is_zero() {
            frame_duration = Duration::from_millis(100);
        }

        let samples = f.into_buffer().into_flat_samples();

        let bytes = glib::Bytes::from(samples.as_slice());
        let layout = samples.layout;

        let texture = gdk::MemoryTexture::new(
            layout.width as i32,
            layout.height as i32,
            gdk::MemoryFormat::R8g8b8a8,
            &bytes,
            layout.height_stride,
        );

        Frame {
            texture: texture.upcast(),
            frame_duration,
        }
    }
}
