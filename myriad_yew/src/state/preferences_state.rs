use std::rc::Rc;

use web_sys::window;
#[cfg(target_arch = "wasm32")]
use yewdux::storage;
use yewdux::{
    prelude::{init_listener, Listener},
    store::{Store, Reducer},
};

fn update_window_dark_state(state: DarkModeState) -> Option<()> {
    log::info!("Setting Dark");

    let window = window()?;
    let document = window.document()?;
    let body = document.body()?;
    let root = body.parent_element()?;
    let class_name = root.class_name();

    let fixed_class_name = class_name.replace("dark", "").replace("light", "");
    let suffix = match state {
        DarkModeState::Auto => "",
        DarkModeState::Light => " light",
        DarkModeState::Dark => " dark",
    };

    root.set_class_name(format!("{fixed_class_name}{suffix}").as_str());
    Some(())
}

#[derive(Default, PartialEq, Eq, Clone, Copy, serde:: Serialize, serde::Deserialize, Debug)]
pub enum DarkModeState {
    #[default]
    Auto,
    Light,
    Dark,
}

impl Store for DarkModeState {
    #[cfg(not(target_arch = "wasm32"))]
    fn new() -> Self {
        init_listener(DarkModeListener);
        let state: DarkModeState = Default::default();
        update_window_dark_state(state);
        state
    }

    #[cfg(target_arch = "wasm32")]

    fn new() -> Self {
        log::info!("Loading Dark Mode State");
        init_listener(DarkModeListener);

        let state: DarkModeState = storage::load(storage::Area::Local)
            .ok()
            .flatten()
            .unwrap_or_default();

        update_window_dark_state(state);
        state
    }

    fn should_notify(&self, other: &Self) -> bool {
        self != other
    }
}

struct DarkModeListener;
impl Listener for DarkModeListener {
    type Store = DarkModeState;

    fn on_change(&mut self, state: Rc<Self::Store>) {
        update_window_dark_state(*state);

        log::info!("Saving dark mode state {state:?}");
        #[cfg(target_arch = "wasm32")]
        {
            use yewdux::storage::save;
            save(state.as_ref(), storage::Area::Local).expect("unable to save state");
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DarkModeNextMessage;

impl Reducer<DarkModeState> for DarkModeNextMessage{
    fn apply(self, state: Rc<DarkModeState>) -> Rc<DarkModeState> {
        use DarkModeState::*;
        match *state{
            Auto => Light,
            Light => Dark,
            Dark => Auto,
        }.into()
    }
}