use capacitor_bindings::device::Device;
use std::rc::Rc;
use yewdux::store::AsyncReducer;
use yewdux::store::Store;

use super::logging::EventLog;
use super::logging::LoggableEvent;
use super::prelude::DeviceUUID;
use crate::state::logging::LogAppInfo;
use crate::state::logging::LogDeviceInfo;
use crate::state::util::get_referrer;
use crate::web::capacitor;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct UserState {
    pub user_id1: Option<DeviceUUID>,
    pub ref_param: Option<String>,
    pub gclid_param: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct UpdateParamsIfNewMessage {
    pub ref_param: Option<String>,
    pub gclid_param: Option<String>,
}

#[yewdux::prelude::async_reducer]
impl AsyncReducer<UserState> for UpdateParamsIfNewMessage {
    /// Mutate state.
    async fn apply(self, state: Rc<UserState>) -> Rc<UserState> {
        log::info!("Updating params if new");
        if state.user_id1.is_some() {
            log::info!("Not new");
            state
        } else {
            log::info!("User is new");
            let mut state = state.as_ref().clone();
            let device_id = capacitor::get_or_log_error_async(Device::get_id).await;
            state.user_id1 = device_id.map(|x| DeviceUUID(x.uuid));
            state.gclid_param = self.gclid_param;
            state.ref_param = self.ref_param;
            log::info!("Params updated");
            let state: Rc<UserState> = state.into();
            UpdateParamsIfNewMessage::try_send_log(state.clone()).await;
            state
        }
    }
}

impl UpdateParamsIfNewMessage {
    async fn try_send_log(state: Rc<UserState>) {
        log::info!("User state has changed");
        if let Some(device_id) = &state.as_ref().user_id1 {
            let referrer = get_referrer();

            let device = LogDeviceInfo::try_get_async().await;
            let app = LogAppInfo::try_get_async().await;

            let language = capacitor::get_or_log_error_async(Device::get_language_tag).await;

            let message = EventLog {
                user_id: device_id.clone(),

                event: LoggableEvent::NewUser {
                    device,
                    app,
                    ref_param: state.ref_param.clone(),
                    gclid: state.gclid_param.clone(),
                    referrer,
                    language: language.map(|x| x.value),
                },
                resent: false,
                severity: super::logging::Severity::Info,
            };
            message.send_log_async().await;
        }
    }
}
