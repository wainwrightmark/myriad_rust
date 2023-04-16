use crate::state::failed_logs_state::FailedLogsState;
use crate::state::prelude::*;
use crate::state::user_state::{UserState, UpdateParamsIfNewMessage};

use web_sys::UrlSearchParams;
use yewdux::prelude::Dispatch;

pub async fn get_url_search_params() -> Option<UrlSearchParams> {
    #[cfg(any(feature = "android", feature = "ios"))]
    {
        let url = capacitor_bindings::app::App::get_launch_url()
            .await
            .ok()??;

        let url = web_sys::Url::new(&url.url).ok()?;
        let params = url.search_params();
        return Some(params);
    }

    #[cfg(not(any(feature = "android", feature = "ios")))]
    {
        use web_sys::window;
        let window = window()?;
        let search = window.location().search().ok()?;
        let params = UrlSearchParams::new_with_str(search.as_str()).ok()?;
        Some(params)
    }
}

pub async fn setup() {
    Dispatch::<FailedLogsState>::new()
        .apply_future(ResentFailedLogsMessage)
        .await;

    #[cfg(any(feature = "android", feature = "ios"))]
    {
        use super::app_redirect;
        app_redirect::subscribe_to_app_url_events().await;
    }

    let url_search_params = get_url_search_params().await;
    let ref_param = url_search_params.clone().and_then(|u| u.get("ref"));
    let gclid_param = url_search_params.clone().and_then(|u| u.get("gclid"));


    Dispatch::<UserState>::new()
        .apply_future(UpdateParamsIfNewMessage {
            ref_param,
            gclid_param,
        })
        .await;

    #[cfg(feature = "android")]
    {
        use capacitor_bindings::status_bar::*;
        crate::web::capacitor::do_or_report_error_async(|| async {
            StatusBar::set_overlays_web_view(SetOverlaysWebViewOptions { overlay: true }).await
        })
        .await;

        match capacitor_bindings::app::App::add_back_button_listener(move |event| {
            if !(event.can_go_back && try_go_back()) {
                crate::web::capacitor::do_or_report_error(capacitor_bindings::app::App::exit_app);
            }
        })
        .await
        {
            Ok(handle) => handle.leak(),
            Err(err) => {
                crate::state::logging::LoggableEvent::try_log_error_message_async(err.to_string())
                    .await;
            }
        }
    }
    #[cfg(feature = "ios")]
    {
        use capacitor_bindings::status_bar::*;
        crate::web::capacitor::do_or_report_error_async(|| async { StatusBar::hide().await }).await;
    }


}

#[cfg(feature = "android")]
/// Goes back, returns true if successful
fn try_go_back() -> bool {
    match web_sys::window() {
        Some(w) => match w.history() {
            Ok(h) => match h.back() {
                Ok(()) => {
                    if w.location().pathname().unwrap_or_default().is_empty() {
                        return false;
                    }

                    return true;
                }
                Err(_) => false,
            },
            Err(_) => false,
        },
        None => false,
    }
}
