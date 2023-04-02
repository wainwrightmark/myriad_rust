use std::num::NonZeroU8;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct Difficulty(pub NonZeroU8);

impl Difficulty {
    pub fn dots(&self) -> &'static str {
        match self.0.get() {
            1 => "🤦",
            2 => "🤦🤦",
            3 => "👼👼👼",
            4 => "👼👼👼👼",
            5 => "👼👼👼👼👼",
            6 => "🤖🤖🤖🤖🤖🤖",
            7 => "🤖🤖🤖🤖🤖🤖🤖",
            8 => "😈😈😈😈😈😈😈😈",
            9 => "😈😈😈😈😈😈😈😈😈",
            _ => panic!("No Difficulty {}", self.0),
        }

        // match self.0.get() {
        //     1 => "🀙",
        //     2 => "🀚",
        //     3 => "🀛",
        //     4 => "🀜",
        //     5 => "🀝",
        //     6 => "🀞",
        //     7 => "🀟",
        //     8 => "🀠",
        //     9 => "🀡",
        //     _ => panic!("No Difficulty {}", self.0),
        // }
    }
}
