use crate::core::prelude::*;
use crate::state::msg::*;
use crate::state::fullstate::*;
use std::rc::Rc;
use yewdux::prelude::*;

pub enum DragMsg{
    MouseDown{coordinate: Coordinate},
    MouseUp{coordinate: Coordinate},
    
    TouchStart{coordinate: Coordinate},
    TouchEnd{coordinate: Coordinate}
}

impl Reducer<DragState> for DragMsg{
    fn apply(&self, state: Rc<DragState>) -> Rc<DragState> {        

        match self{
            DragMsg::MouseDown { coordinate } => {
                log::debug!("Mouse Down");
                DragState{mouse_coordinate: Some(coordinate.clone()), ..Default::default()}.into()
            },
            DragMsg::MouseUp { coordinate } => {
                
                if let Some(start) = state.mouse_coordinate{

                    if &start == coordinate{
                        log::debug!("Mouse Up - Click");
                        Dispatch::new().apply( Msg::Move { coordinate: start });
                    }
                    else {
                        log::debug!("Mouse Up - Rotate");
                        Dispatch::<FullState>::new().apply(Msg::FlipAndRotateRelative{rotate:1, flip: false});    
                    }

                                    
                }
                else {
                    log::debug!("Mouse Up Nothing");                    
                }
                DragState{..Default::default()}.into()
            },
            
            DragMsg::TouchStart { coordinate } => {
                log::debug!("Touch End");
                DragState{touch_coordinate: Some(coordinate.clone()), ..Default::default()}.into()
            },
            DragMsg::TouchEnd { coordinate } => {
                
                if let Some(start) = state.touch_coordinate{

                    if &start == coordinate{
                        log::debug!("Touch Start - Click");
                        Dispatch::new().apply( Msg::Move { coordinate: start });
                    }
                    else {
                        log::debug!("Touch Start - Rotate");
                        Dispatch::<FullState>::new().apply(Msg::FlipAndRotateRelative{rotate:1, flip: false});    
                    }

                                    
                }
                else {
                    log::debug!("Touch Start Nothing");                    
                }
                DragState{..Default::default()}.into()
            },

        }
    }
}

#[derive(PartialEq, Store, Clone, Default)]
pub struct DragState {
        pub mouse_coordinate: Option<Coordinate>,
        pub touch_coordinate: Option<Coordinate>,
}