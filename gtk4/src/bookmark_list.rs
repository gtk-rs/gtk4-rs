// Take a look at the license at the top of the repository in the LICENSE file.

use crate::BookmarkList;
use glib::{translate::*, ToValue};

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
        BookmarkListBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`BookmarkList`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BookmarkListBuilder {
    attributes: Option<String>,
    filename: Option<String>,
    io_priority: Option<i32>,
}

impl BookmarkListBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> BookmarkList {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref attributes) = self.attributes {
            properties.push(("attributes", attributes));
        }
        if let Some(ref filename) = self.filename {
            properties.push(("filename", filename));
        }
        if let Some(ref io_priority) = self.io_priority {
            properties.push(("io-priority", io_priority));
        }
        glib::Object::new::<BookmarkList>(&properties)
    }

    pub fn attributes(mut self, attributes: &str) -> Self {
        self.attributes = Some(attributes.to_string());
        self
    }

    pub fn filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_string());
        self
    }

    pub fn io_priority(mut self, io_priority: glib::Priority) -> Self {
        self.io_priority = Some(io_priority.into_glib());
        self
    }
}
