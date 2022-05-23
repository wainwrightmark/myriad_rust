use crate::core::parser::*;
use crate::core::prelude::*;
use crate::state::prelude::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CircleType {
    Disabled,
    LegalMove,
    LastPosition,
    IntermediatePosition { next: Coordinate },
}

impl CircleType {
    pub fn get_cursor(&self) -> &str {
        match self {
            CircleType::Disabled => "not-allowed",
            CircleType::LastPosition => "crosshair",
            CircleType::IntermediatePosition { next: _ } => "pointer",
            CircleType::LegalMove => "pointer",
        }
    }

    pub fn get_color(&self) -> &str {
        match self {
            CircleType::Disabled => "grey",
            CircleType::LastPosition => "blue",
            CircleType::IntermediatePosition { next: _ } => "green",
            CircleType::LegalMove => "black",
        }
    }
}

impl ChosenPositionsState {
    pub fn get_circle_type(
        &self,
        coordinate: &Coordinate,
        board: std::rc::Rc<Board<3,3>>,
    ) -> CircleType {
        if let Some(position) = self.positions.iter().position(|c| c == coordinate) {
            if let Some(next) = self.positions.get(position + 1) {
                return CircleType::IntermediatePosition { next: *next };
            } else {
                return CircleType::LastPosition;
            }
        }

        if let Some(last) = self.positions.last() {
            if !last.is_adjacent(*coordinate) {
                return CircleType::Disabled;
            }
        }

        let mut letters = self
            .positions
            .iter()
            .chain(std::iter::once(coordinate))
            .map(|c| board.get_letter_at_coordinate(c))
            .peekable();

        let parse_result = parse_and_evaluate(&mut letters);

        match parse_result {
            Ok(_) => CircleType::LegalMove,
            Err(parse_fail) => match parse_fail {
                ParseFail::PartialSuccess => CircleType::LegalMove,
                ParseFail::Failure => CircleType::Disabled,
            },
        }
    }
}
