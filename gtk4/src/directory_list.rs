// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DirectoryList;
use glib::translate::*;
use glib::{Cast, IsA, ToValue};

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
    /// This method returns an instance of [`DirectoryListBuilder`] which can be used to create [`DirectoryList`] objects.
    pub fn builder() -> DirectoryListBuilder {
        DirectoryListBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct DirectoryListBuilder {
    attributes: Option<String>,
    file: Option<gio::File>,
    io_priority: Option<i32>,
    monitored: Option<bool>,
}

impl DirectoryListBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> DirectoryList {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref attributes) = self.attributes {
            properties.push(("attributes", attributes));
        }
        if let Some(ref file) = self.file {
            properties.push(("file", file));
        }
        if let Some(ref io_priority) = self.io_priority {
            properties.push(("io-priority", io_priority));
        }
        if let Some(ref monitored) = self.monitored {
            properties.push(("monitored", monitored));
        }
        glib::Object::new::<DirectoryList>(&properties)
            .expect("Failed to create an instance of DirectoryList")
    }

    pub fn attributes(mut self, attributes: &str) -> Self {
        self.attributes = Some(attributes.to_string());
        self
    }

    pub fn file<P: IsA<gio::File>>(mut self, file: &P) -> Self {
        self.file = Some(file.clone().upcast());
        self
    }

    pub fn io_priority(mut self, io_priority: glib::Priority) -> Self {
        self.io_priority = Some(io_priority.into_glib());
        self
    }

    pub fn monitored(mut self, monitored: bool) -> Self {
        self.monitored = Some(monitored);
        self
    }
}
