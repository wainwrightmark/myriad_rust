use crate::state::GOALSIZE;
use num::ToPrimitive;
use serde::*;

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
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
