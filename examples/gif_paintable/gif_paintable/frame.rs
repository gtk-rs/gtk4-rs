use std::time::Duration;

use gtk::prelude::*;
use gtk::{gdk, glib};

pub struct Frame {
    pub width: i32,
    pub height: i32,
    pub texture: gdk::Paintable,
    pub frame_duration: Duration,
}

impl Frame {
    pub fn new(f: image::Frame) -> Self {
        let mut frame_duration = Duration::from(f.delay());

        // convention is to use 100 milliseconds duration if it is defined as 0.
        if frame_duration.is_zero() {
            frame_duration = Duration::from_millis(100);
        }

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
