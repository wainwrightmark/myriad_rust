use std::future::Future;

use capacitor_bindings::helpers::Error;

use crate::state::logging;

pub fn do_or_report_error<Fut: Future<Output = Result<(), Error>>, F: Fn() -> Fut + 'static>(f: F) {
    yew::platform::spawn_local(async move { do_or_report_error_async(f).await })
}

pub async fn do_or_report_error_async<
    Fut: Future<Output = Result<(), Error>>,
    F: Fn() -> Fut + 'static,
>(
    f: F,
) {
    let r = f().await;

    match r {
        Ok(_) => {}
        Err(err) => {
            log::error!("{err:?}");
            logging::LoggableEvent::try_log_error_message_async(err.to_string()).await;
        }
    }
}

pub async fn get_or_log_error_async<
    T,
    Fut: Future<Output = Result<T, Error>>,
    F: Fn() -> Fut + 'static,
>(
    f: F,
) -> Option<T> {
    let r = f().await;

    match r {
        Ok(data) => Some(data),
        Err(err) => {
            logging::LoggableEvent::try_log_error_message_async(err.to_string()).await;
            None
        }
    }
}
