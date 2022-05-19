use crate::state::prelude::*;
use crate::core::prelude::*;
use crate::core::parser::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CircleType{
    Disabled,
    LegalMove,
    LastPosition,
    IntermediatePosition{next: Coordinate}
}

impl CircleType{
    pub fn get_cursor(&self)-> &str{
        match self{
            CircleType::Disabled => "not-allowed",
            CircleType::LastPosition => "crosshair",
            CircleType::IntermediatePosition{next:_} => "pointer",
            CircleType::LegalMove  => "pointer",
        }
    }

    pub fn get_color(&self) -> &str{
        match self{
            CircleType::Disabled => "grey",
            CircleType::LastPosition => "blue",
            CircleType::IntermediatePosition{next:_} => "green",
            CircleType::LegalMove  => "black",
        }
    }
}

impl FullState{
    pub fn get_circle_type(&self, coordinate: &Coordinate) -> CircleType {
        
        if let Some(position) = self.chosen_positions.positions.iter().position(|c|c == coordinate){

            if let Some(next)  = self.chosen_positions.positions.get(position + 1){
                return  CircleType::IntermediatePosition { next: next.clone() };
            }
            else{
                return  CircleType::LastPosition;
            }
        }

        if let Some(last) = self.chosen_positions.positions.last(){
            if !last.is_adjacent(coordinate){
                return  CircleType::Disabled;
            }
        }

        let mut letters = self.chosen_positions.positions
        .iter()
        .chain(std::iter::once(coordinate))
        .map(|c| self.board.get_letter_at_coordinate(c))
        .peekable();

        let parse_result =parse_and_evaluate(&mut letters);

        return match  parse_result{
                Ok(_) => CircleType::LegalMove,
                Err(parse_fail) => match parse_fail {
                    ParseFail::PartialSuccess => CircleType::LegalMove,
                    ParseFail::Failure => CircleType::Disabled,
                } ,
            };
    }
}

