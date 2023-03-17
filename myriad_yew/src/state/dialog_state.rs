use serde::*;
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct DialogState {
    pub congratulations_dialog_type: Option<CongratsDialogType>,
    pub history_dialog_type: Option<HistoryDialogType>,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum CongratsDialogType {
    Challenge,
    OneHundred,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default)]
pub struct HistoryDialogType {}
