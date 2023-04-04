use crate::state::prelude::*;
use myriad::parser::*;
use myriad::prelude::*;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum CircleType {
    Disabled,
    LegalMove,
    LastPosition,
    IntermediatePosition {
        next: Tile<GRID_COLUMNS, GRID_ROWS>,
    },
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
            CircleType::Disabled => "var(--circle-disabled)",
            CircleType::LastPosition => "var(--circle-last)",
            CircleType::IntermediatePosition { next: _ } => "var(--circle-intermediate)",
            CircleType::LegalMove => "var(--circle-legal-move)",
        }
    }
}

impl ChosenPositionsState {
    pub fn get_circle_type(
        &self,
        coordinate: &Tile<GRID_COLUMNS, GRID_ROWS>,
        board: &Board<GRID_COLUMNS, GRID_ROWS, 9>,
    ) -> CircleType {
        if let Some(position) = self.positions.iter().position(|c| c == coordinate) {
            if let Some(next) = self.positions.get(position + 1) {
                return CircleType::IntermediatePosition { next: *next };
            } else {
                return CircleType::LastPosition;
            }
        }

        if let Some(last) = self.positions.last() {
            if !last.is_adjacent_to(coordinate) {
                return CircleType::Disabled;
            }
        }

        let mut letters = self
            .positions
            .iter()
            .chain(std::iter::once(coordinate))
            .map(|c| board[*c])
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
