use serde::*;
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct DialogState {
    pub congratulations_dialog_type: Option<CongratsDialogType>,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum CongratsDialogType {
    OneHundred,
}

