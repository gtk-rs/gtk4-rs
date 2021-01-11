// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{FileChooserAction, FileChooserExt, FileChooserNative, FileFilter, Window};
use glib::object::{Cast, IsA};
use glib::ToValue;

#[derive(Clone, Default)]
pub struct FileChooserNativeBuilder {
    accept_label: Option<String>,
    cancel_label: Option<String>,
    modal: Option<bool>,
    title: Option<String>,
    transient_for: Option<Window>,
    visible: Option<bool>,
    action: Option<FileChooserAction>,
    create_folders: Option<bool>,
    filters: Vec<FileFilter>,
    select_multiple: Option<bool>,
}

impl FileChooserNativeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> FileChooserNative {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref accept_label) = self.accept_label {
            properties.push(("accept-label", accept_label));
        }
        if let Some(ref cancel_label) = self.cancel_label {
            properties.push(("cancel-label", cancel_label));
        }
        if let Some(ref modal) = self.modal {
            properties.push(("modal", modal));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref transient_for) = self.transient_for {
            properties.push(("transient-for", transient_for));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref action) = self.action {
            properties.push(("action", action));
        }
        if let Some(ref create_folders) = self.create_folders {
            properties.push(("create-folders", create_folders));
        }
        if let Some(ref select_multiple) = self.select_multiple {
            properties.push(("select-multiple", select_multiple));
        }
        let file_chooser = glib::Object::new::<FileChooserNative>(&properties)
            .expect("Failed to create FileChooserNative.");
        for filter in self.filters {
            file_chooser.add_filter(&filter);
        }
        file_chooser
    }

    pub fn accept_label(mut self, accept_label: &str) -> Self {
        self.accept_label = Some(accept_label.to_string());
        self
    }

    pub fn cancel_label(mut self, cancel_label: &str) -> Self {
        self.cancel_label = Some(cancel_label.to_string());
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn transient_for<P: IsA<Window>>(mut self, transient_for: &P) -> Self {
        self.transient_for = Some(transient_for.clone().upcast());
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn action(mut self, action: FileChooserAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn create_folders(mut self, create_folders: bool) -> Self {
        self.create_folders = Some(create_folders);
        self
    }

    pub fn add_filter(mut self, filter: FileFilter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn select_multiple(mut self, select_multiple: bool) -> Self {
        self.select_multiple = Some(select_multiple);
        self
    }
}
