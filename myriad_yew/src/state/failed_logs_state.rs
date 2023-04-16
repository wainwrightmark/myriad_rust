use std::rc::Rc;
use yewdux::prelude::async_reducer;
use yewdux::prelude::Dispatch;
use yewdux::store::AsyncReducer;
use yewdux::store::Reducer;
use yewdux::store::Store;

use crate::state::user_state::UserState;

use super::logging::EventLog;
use super::prelude::*;
#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct FailedLogsState {
    pub logs: Vec<LoggableEvent>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ResentFailedLogsMessage;

#[async_reducer]
impl AsyncReducer<FailedLogsState> for ResentFailedLogsMessage {
    async fn apply(self, state: Rc<FailedLogsState>) -> Rc<FailedLogsState> {
        //log::info!("Checking for failed logs");
        if state.logs.is_empty() {
            return state;
        }
        let user = Dispatch::<UserState>::new().get();
        let Some(user_id) = user.as_ref().user_id1.clone() else{
            log::error!("User Id not set");
            return state;
        };

        log::info!("{} failed logs found", state.logs.len());

        for event in state.logs.iter() {
            let log = EventLog::new_resent(user_id.clone(), event.clone());
            log.send_log_async().await;
        }

        FailedLogsState::default().into()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct LogFailedMessage(pub LoggableEvent);

impl Reducer<FailedLogsState> for LogFailedMessage {
    fn apply(self, state: Rc<FailedLogsState>) -> Rc<FailedLogsState> {
        let mut new_state = (*state).clone();
        new_state.logs.push(self.0);

        new_state.into()
    }
}
