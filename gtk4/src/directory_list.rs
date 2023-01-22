// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, DirectoryList};
use glib::translate::*;

impl DirectoryList {
    #[doc(alias = "gtk_directory_list_get_io_priority")]
    #[doc(alias = "get_io_priority")]
    pub fn io_priority(&self) -> glib::Priority {
        unsafe {
            from_glib(ffi::gtk_directory_list_get_io_priority(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_directory_list_set_io_priority")]
    pub fn set_io_priority(&self, io_priority: glib::Priority) {
        unsafe {
            ffi::gtk_directory_list_set_io_priority(self.to_glib_none().0, io_priority.into_glib());
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DirectoryList`] objects.
    ///
    /// This method returns an instance of [`DirectoryListBuilder`](crate::builders::DirectoryListBuilder) which can be used to create [`DirectoryList`] objects.
    pub fn builder() -> DirectoryListBuilder {
        DirectoryListBuilder::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DirectoryList`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DirectoryListBuilder {
    builder: glib::object::ObjectBuilder<'static, DirectoryList>,
}

impl DirectoryListBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DirectoryList {
        self.builder.build()
    }

    pub fn attributes(self, attributes: &str) -> Self {
        Self {
            builder: self.builder.property("attributes", attributes),
        }
    }

    pub fn file(self, file: &impl IsA<gio::File>) -> Self {
        Self {
            builder: self.builder.property("file", file),
        }
    }

    pub fn io_priority(self, io_priority: glib::Priority) -> Self {
        Self {
            builder: self
                .builder
                .property("io-priority", io_priority.into_glib()),
        }
    }

    pub fn monitored(self, monitored: bool) -> Self {
        Self {
            builder: self.builder.property("monitored", monitored),
        }
    }
}
