use std::ops::Not;

use capacitor_bindings::{
    app::AppInfo,
    device::{Device, DeviceInfo, OperatingSystem, Platform},
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::EnumDiscriminants;
use wasm_bindgen_futures::spawn_local;
use yewdux::prelude::Dispatch;

use crate::{
    state::prelude::*,
    web::{capacitor, prelude::*},
};

use super::user_state::UserState;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Debug)]
#[serde(transparent)]
pub struct DeviceUUID(pub String);

// cSpell:ignore xaat

/// This token can only be used to ingest data into our bucket
const API_TOKEN: &str = "xaat-3a2dca4d-ecbe-4b81-8d1c-1ef103b3ff42";

#[derive(Debug, Clone, Serialize)]
pub struct EventLog {
    pub user_id: DeviceUUID,
    #[serde(skip_serializing_if = "is_false")]
    pub resent: bool,
    pub event: LoggableEvent,
    #[serde(skip_serializing_if = "is_info_or_lower")]
    pub severity: Severity,
}

fn is_false(b: &bool) -> bool {
    !b
}

fn is_info_or_lower(severity: &Severity) -> bool {
    severity != &Severity::Warn && severity != &Severity::Error
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warn,
    Error,
}

impl EventLog {
    pub fn new_resent(user_id: DeviceUUID, event: LoggableEvent) -> Self {
        let severity = event.get_severity();
        Self {
            user_id,
            resent: true,
            event,
            severity,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogAppInfo {
    build: String,
    version: String,
}

impl From<AppInfo> for LogAppInfo {
    fn from(value: AppInfo) -> Self {
        Self {
            build: value.build,
            version: value.version,
        }
    }
}

impl LogAppInfo {
    pub async fn try_get_async() -> Option<LogAppInfo> {
        #[cfg(any(feature = "android", feature = "ios"))]
        {
            crate::web::capacitor::get_or_log_error_async(capacitor_bindings::app::App::get_info)
                .await
                .map(|x| x.into())
        }
        #[cfg(not(any(feature = "android", feature = "ios")))]
        {
            None
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogDeviceInfo {
    pub name: Option<String>,
    pub model: String,
    pub platform: Platform,
    pub os: OperatingSystem,
    pub os_version: String,
    pub manufacturer: String,
    pub is_virtual: bool,
    pub web_view_version: Option<String>,
}

impl From<DeviceInfo> for LogDeviceInfo {
    fn from(d: DeviceInfo) -> Self {
        Self {
            name: d.name,
            model: d.model,
            platform: d.platform,
            os: d.operating_system,
            os_version: d.os_version,
            manufacturer: d.manufacturer,
            is_virtual: d.is_virtual,
            web_view_version: d.web_view_version,
        }
    }
}

impl LogDeviceInfo {
    pub async fn try_get_async() -> Option<LogDeviceInfo> {
        capacitor::get_or_log_error_async(Device::get_info)
            .await
            .map(|x| x.into())
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, EnumDiscriminants)]
#[serde(tag = "type")]
pub enum LoggableEvent {
    NewUser {
        ref_param: Option<String>,
        referrer: Option<String>,
        gclid: Option<String>,
        language: Option<String>,
        device: Option<LogDeviceInfo>,
        app: Option<LogAppInfo>,
    },
    NewGame {
        today: bool,
        board: String,
    },
    ClickShare,
    ShareOn {
        platform: String,
    },
    GameComplete {
        board: String,
    },

    // Achievement {
    //     achievement: Achievement,
    // },
    ReceivedShare {
        ref_param: Option<String>,
        referrer: Option<String>,
        spread_id: Option<String>,
        img_id: Option<String>,
    },

    Warn {
        message: String,
    },
    Error {
        message: String,
    },

    Internal {
        message: String,
    },
}

impl LoggableEvent {
    pub async fn try_log_error_message_async(message: String) {
        log::error!("{}", message);
        if !Self::should_ignore_error(&message) {
            let event = LoggableEvent::Error { message };
            Self::try_log_async(event).await
        }
    }

    pub fn should_ignore_error(error: &str) -> bool {
        const ERRORS_TO_IGNORE: &[&'static str] = &[
            "Js Exception: Notifications not supported in this browser.",
            "Js Exception: Browser does not support the vibrate API",
            "Js Exception: Abort due to cancellation of share.",
            "Js Exception: Share canceled",
            "Js Exception: Share API not available in this browser",
        ];
        if ERRORS_TO_IGNORE.contains(&error) {
            return true;
        }

        false
    }

    pub async fn try_log_error_async(err: impl Into<anyhow::Error>) {
        Self::try_log_error_message_async(err.into().to_string()).await
    }

    pub fn try_log_error(err: impl Into<anyhow::Error> + 'static) {
        spawn_local(async move { Self::try_log_error_async(err).await })
    }

    /// Either logs the message or sends it to be retried later
    pub async fn try_log_async(data: impl Into<Self>) {
        let user = Dispatch::<UserState>::new().get();
        let event = data.into();
        let severity = event.get_severity();
        if let Some(user_id) = &user.user_id1 {
            let message = EventLog {
                event,
                user_id: user_id.clone(),
                resent: false,
                severity,
            };
            message.send_log_async().await;
        } else {
            Dispatch::<FailedLogsState>::new().apply(LogFailedMessage(event));
            log::error!("User Id not set");
        }
    }

    pub fn try_log(data: impl Into<Self> + 'static) {
        wasm_bindgen_futures::spawn_local(async move { Self::try_log_async(data).await });
    }

    pub fn get_severity(&self) -> Severity {
        match self {
            LoggableEvent::Warn { .. } => Severity::Warn,
            LoggableEvent::Error { .. } => Severity::Error,
            _ => Severity::Info,
        }
    }


    pub fn new_share(
        ref_param: Option<String>,
        spread_id: Option<String>,
        img_id: Option<String>,
    ) -> Self {
        let referrer = get_referrer();
        Self::ReceivedShare {
            referrer,
            ref_param,
            spread_id,
            img_id,
        }
    }
}

impl EventLog {
    pub async fn send_log_async(self) {
        Self::log(self).await
    }

    async fn try_log<T: Serialize>(data: &T) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.axiom.co/v1/datasets/myriadusage/ingest")
            // .header("Authorization", format!("Bearer {API_TOKEN}"))
            .bearer_auth(API_TOKEN)
            .header("Content-Type", "application/json")
            .json(&[data])
            .send()
            .await?;

        res.error_for_status().map(|_| ())
    }

    async fn log(data: Self) {
        let r = Self::try_log(&data).await;
        if let Err(err) = r {
            log::error!("Failed to log: {}", err);
            Dispatch::<FailedLogsState>::new().apply(LogFailedMessage(data.event));
        } else {
            let discriminant: LoggableEvent = data.event;
            log::debug!("Log {discriminant:?} sent successfully",);
        }
    }
}
