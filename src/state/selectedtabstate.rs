use crate::state::prelude::*;
use num::ToPrimitive;
use serde::*;
use yewdux::prelude::{Reducer, Store};

pub struct NumberFoundMsg {
    pub number: i32,
}

impl Reducer<SelectedTabState> for NumberFoundMsg {
    fn apply(&self, state: std::rc::Rc<SelectedTabState>) -> std::rc::Rc<SelectedTabState> {
        state.number_found(self.number).into()
    }
}

pub struct TabSelectedMsg {
    pub index: usize,
}

impl Reducer<SelectedTabState> for TabSelectedMsg {
    fn apply(&self, state: std::rc::Rc<SelectedTabState>) -> std::rc::Rc<SelectedTabState> {
        state.tab_clicked(self.index).into()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default, Store)]
#[store(storage = "local")] // can also be "session"
pub struct SelectedTabState {
    pub index: usize,
    pub locked: bool,
}

impl SelectedTabState {
    pub fn tab_clicked(self, tab: usize) -> Self {
        if self.index == tab {
            if self.locked {
                SelectedTabState {
                    index: tab,
                    locked: false,
                }
            } else {
                SelectedTabState {
                    index: tab,
                    locked: true,
                }
            }
        } else {
            SelectedTabState {
                index: tab,
                locked: self.locked,
            }
        }
    }

    pub fn number_found(self, number: i32) -> Self {
        if self.locked {
            self
        } else {
            let i = (number - 1) / GOALSIZE;
            SelectedTabState {
                index: i.to_usize().unwrap(),
                locked: false,
            }
        }
    }
}
