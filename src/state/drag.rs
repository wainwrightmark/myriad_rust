use crate::core::prelude::*;
use crate::state::msg::*;
use std::rc::Rc;
use yewdux::prelude::*;

pub enum DragMsg {
    MouseDown { coordinate: Coordinate },
    MouseUp { coordinate: Coordinate },

    TouchStart { coordinate: Coordinate },
    TouchEnd { coordinate: Coordinate },
}

#[derive(PartialEq, Store, Clone, Default)]
pub struct DragState {
    pub mouse_coordinate: Option<Coordinate>,
    pub touch_coordinate: Option<Coordinate>,
}

impl Reducer<DragState> for DragMsg {
    fn apply(&self, state: Rc<DragState>) -> Rc<DragState> {
        match self {
            DragMsg::MouseDown { coordinate } => DragState {
                mouse_coordinate: Some(coordinate.clone()),
                ..Default::default()
            }
            .into(),
            DragMsg::MouseUp { coordinate: end } => {
                if let Some(start) = state.mouse_coordinate {
                    if let Some(msg) =
                        find_message(start, end.clone(), Coordinate { row: 2, column: 2 })
                    {
                        Dispatch::new().apply(msg);
                    }
                }

                DragState {
                    ..Default::default()
                }
                .into()
            }
            _ => state
            // DragMsg::TouchStart { coordinate } => DragState {
            //     touch_coordinate: Some(coordinate.clone()),
            //     ..Default::default()
            // }
            // .into(),
            // DragMsg::TouchEnd { coordinate: end } => {
            //     if let Some(start) = state.touch_coordinate {
            //         if let Some(msg) =
            //             find_message(start, end.clone(), Coordinate { row: 2, column: 2 })
            //         {
            //             Dispatch::new().apply(msg);
            //         }
            //     }
            //     DragState {
            //         ..Default::default()
            //     }
            //     .into()
            // }
        }
    }
}

fn find_message(start: Coordinate, end: Coordinate, max_coordinate: Coordinate) -> Option<Msg> {
    if start == end {
        return Msg::Move { coordinate: start }.into();
    } else {
        let start_type = CoordinateType::from(start);
        let end_type = CoordinateType::from(end);

        match start_type {
            CoordinateType::Corner if matches!(end_type, CoordinateType::Corner) => {
                for rotate in 1..4 {
                    if start.rotate_and_flip(max_coordinate, rotate, false) == end {
                        return Msg::FlipAndRotateRelative {
                            rotate,
                            flip: false,
                        }
                        .into();
                    }
                }
            }
            CoordinateType::Edge if matches!(end_type, CoordinateType::Edge) => {
                for rotate in 0..4 {
                    if start.rotate_and_flip(max_coordinate, rotate, true) == end {
                        return Msg::FlipAndRotateRelative { rotate, flip: true }.into();
                    }
                }
            }
            _ => return None,
        }
    }

    None
}

enum CoordinateType {
    Center,
    Corner,
    Edge,
}

impl From<Coordinate> for CoordinateType {
    fn from(c: Coordinate) -> Self {
        if c.column == 1 && c.row == 1 {
            CoordinateType::Center
        } else if c.column % 2 == 0 && c.row % 2 == 0 {
            CoordinateType::Corner
        } else {
            CoordinateType::Edge
        }
    }
}
