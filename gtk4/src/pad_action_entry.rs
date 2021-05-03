// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PadActionType;

#[derive(Debug, Clone)]
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
    ) -> PadActionEntry {
        assert_initialized_main_thread!();
        PadActionEntry {
            type_,
            index,
            mode,
            label: label.to_owned(),
            action_name: action_name.to_owned(),
        }
    }

    #[doc(alias = "get_type")]
    pub fn type_(&self) -> PadActionType {
        self.type_
    }

    #[doc(alias = "get_index")]
    pub fn index(&self) -> i32 {
        self.index
    }

    #[doc(alias = "get_mode")]
    pub fn mode(&self) -> i32 {
        self.mode
    }

    #[doc(alias = "get_label")]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[doc(alias = "get_action_name")]
    pub fn action_name(&self) -> &str {
        &self.action_name
    }
}
