// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{LineCap, LineJoin, Stroke};

impl Stroke {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct a [`Stroke`].
    ///
    /// This method returns an instance of
    /// [`StrokeBuilder`](crate::builders::StrokeBuilder) which can be used to
    /// create a [`Stroke`].
    pub fn builder(line_width: f32) -> StrokeBuilder {
        assert_initialized_main_thread!();
        StrokeBuilder(Stroke::new(line_width))
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct a [`Stroke`].
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct StrokeBuilder(Stroke);

impl StrokeBuilder {
    pub fn dash(self, dash: &[f32]) -> Self {
        self.0.set_dash(dash);
        self
    }

    pub fn dash_offset(self, dash_offset: f32) -> Self {
        self.0.set_dash_offset(dash_offset);
        self
    }

    pub fn line_cap(self, line_cap: LineCap) -> Self {
        self.0.set_line_cap(line_cap);
        self
    }

    pub fn line_join(self, line_join: LineJoin) -> Self {
        self.0.set_line_join(line_join);
        self
    }

    pub fn miter_limit(self, miter_limit: f32) -> Self {
        self.0.set_miter_limit(miter_limit);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Stroke`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Stroke {
        self.0
    }
}
