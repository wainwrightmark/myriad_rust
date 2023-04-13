use std::rc::Rc;

use myriad::prelude::Difficulty;
use yewdux::store::{Reducer, Store};

#[derive(Clone, PartialEq, Eq, Debug, Default, Store)]
pub enum InfoBarState {
    #[default]
    None,
    ValidNumber(i32),
    InvalidNumber(i32),
    Equation(String),
    Difficulty(Difficulty),
}

impl InfoBarState {
    pub fn text(&self) -> String {
        match self {
            InfoBarState::None => "".to_string(),
            InfoBarState::ValidNumber(x) => x.to_string(),
            InfoBarState::InvalidNumber(x) => x.to_string(),
            InfoBarState::Equation(x) => x.clone(),
            InfoBarState::Difficulty(d) => d.dots().to_string(),
        }
    }

    pub fn text_color(&self) -> &'static str {
        match self {
            InfoBarState::None => "var(--infobar-none)",
            InfoBarState::ValidNumber(_) => "var(--infobar-valid)",
            InfoBarState::InvalidNumber(_) => "var(--infobar-invalid)",
            InfoBarState::Equation(_) => "var(--infobar-equation)",
            InfoBarState::Difficulty(_) => "var(--infobar-difficulty)",
        }
    }

    pub fn font_size(&self) -> &'static str {
        match self {
            InfoBarState::Difficulty(d) => {
                if d.0.get() <= 4 {
                    "60px"
                } else {
                    "30px"
                }
            }
            _ => "60px",
        }
    }

    pub fn line_height(&self) -> &'static str {
        match self {
            InfoBarState::Difficulty(d) => {
                if d.0.get() <= 4 {
                    "2"
                } else {
                    "4"
                }
            }
            _ => "2",
        }
    }
}

pub struct InfoBarSetMessage(pub InfoBarState);

impl Reducer<InfoBarState> for InfoBarSetMessage {
    fn apply(self, mut state: Rc<InfoBarState>) -> std::rc::Rc<InfoBarState> {
        let s = Rc::make_mut(&mut state);
        *s = self.0;

        state
    }
}
