// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PadActionType;

#[derive(Debug, Clone)]
#[doc(alias = "GtkPadActionEntry")]
pub struct PadActionEntry {
    type_: PadActionType,
    index: i32,
    mode: i32,
    label: String,
    action_name: String,
}

impl PadActionEntry {
    pub fn new(
        type_: PadActionType,
        index: i32,
        mode: i32,
        label: &str,
        action_name: &str,
    ) -> Self {
        assert_initialized_main_thread!();
        Self {
            type_,
            index,
            mode,
            label: label.to_owned(),
            action_name: action_name.to_owned(),
        }
    }

    pub fn type_(&self) -> PadActionType {
        self.type_
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn mode(&self) -> i32 {
        self.mode
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn action_name(&self) -> &str {
        &self.action_name
    }
}
