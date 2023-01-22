// Take a look at the license at the top of the repository in the LICENSE file.

use crate::BookmarkList;
use glib::translate::*;

impl BookmarkList {
    #[doc(alias = "gtk_bookmark_list_get_io_priority")]
    #[doc(alias = "get_io_priority")]
    pub fn io_priority(&self) -> glib::Priority {
        unsafe {
            from_glib(ffi::gtk_bookmark_list_get_io_priority(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_bookmark_list_set_io_priority")]
    pub fn set_io_priority(&self, io_priority: glib::Priority) {
        unsafe {
            ffi::gtk_bookmark_list_set_io_priority(self.to_glib_none().0, io_priority.into_glib());
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`BookmarkList`] objects.
    ///
    /// This method returns an instance of [`BookmarkListBuilder`](crate::builders::BookmarkListBuilder) which can be used to create [`BookmarkList`] objects.
    pub fn builder() -> BookmarkListBuilder {
        BookmarkListBuilder::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`BookmarkList`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BookmarkListBuilder {
    builder: glib::object::ObjectBuilder<'static, BookmarkList>,
}

impl BookmarkListBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> BookmarkList {
        self.builder.build()
    }

    pub fn attributes(self, attributes: &str) -> Self {
        Self {
            builder: self.builder.property("attributes", attributes),
        }
    }

    pub fn filename(self, filename: &str) -> Self {
        Self {
            builder: self.builder.property("filename", filename),
        }
    }

    pub fn io_priority(self, io_priority: glib::Priority) -> Self {
        Self {
            builder: self
                .builder
                .property("io-priority", io_priority.into_glib()),
        }
    }
}
