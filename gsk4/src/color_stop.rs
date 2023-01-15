// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::RGBA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskColorStop")]
    pub struct ColorStop(BoxedInline<ffi::GskColorStop>);
}

impl ColorStop {
    #[inline]
    pub fn new(offset: f32, color: RGBA) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            Self::unsafe_from(ffi::GskColorStop {
                offset,
                color: *color.to_glib_none().0,
            })
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ColorStop`] objects.
    ///
    /// This method returns an instance of [`ColorStopBuilder`](crate::builders::ColorStopBuilder) which can be used to create [`ColorStop`] objects.
    pub fn builder() -> ColorStopBuilder {
        ColorStopBuilder::default()
    }

    #[inline]
    pub fn offset(&self) -> f32 {
        self.inner.offset
    }

    #[inline]
    pub fn color(&self) -> &RGBA {
        unsafe { &*(&self.inner.color as *const gdk::ffi::GdkRGBA as *const RGBA) }
    }
}

impl fmt::Debug for ColorStop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ColorStop")
            .field("offset", &self.offset())
            .field("color", &self.color())
            .finish()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ColorStop`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ColorStopBuilder(Vec<ColorStop>);

impl ColorStopBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ColorStopBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn at(mut self, offset: f32, color: RGBA) -> Self {
        self.0.push(ColorStop::new(offset, color));
        self
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ColorStop`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Vec<ColorStop> {
        self.0
    }
}

#[test]
fn color_stop_builder() {
    let t1 = ColorStop::builder()
        .at(0.0, gdk::RGBA::RED)
        .at(0.333, gdk::RGBA::RED)
        .at(0.667, gdk::RGBA::BLUE)
        .at(1.0, gdk::RGBA::RED)
        .build();

    let t2 = &[
        ColorStop::new(0.0, gdk::RGBA::RED),
        ColorStop::new(0.333, gdk::RGBA::RED),
        ColorStop::new(0.667, gdk::RGBA::BLUE),
        ColorStop::new(1.0, gdk::RGBA::RED),
    ];

    assert_eq!(t1.len(), t2.len(), "Arrays don't have the same length");
    assert!(
        &t1.iter()
            .zip(t2.iter())
            .all(|(a, b)| a.offset() == b.offset() && a.color() == b.color()),
        "Arrays are not equal"
    );
}
