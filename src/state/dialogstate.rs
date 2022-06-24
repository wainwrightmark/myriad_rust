use serde::*;
use yewdux::prelude::*;

#[derive(PartialEq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct DialogState{
    pub dialog_type: Option<DialogType>
}

#[derive(PartialEq,  Clone, Copy, Serialize, Deserialize)]
pub enum DialogType {
    Challenge,
    OneHundred
}
