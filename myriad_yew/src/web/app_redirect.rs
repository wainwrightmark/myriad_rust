#[cfg(any(feature = "android", feature = "ios"))]
pub async fn subscribe_to_app_url_events() {
    use capacitor_bindings::app::App;
    //use crate::state::logging::LoggableEvent;
    // LoggableEvent::try_log(LoggableEvent::Internal { message: format!("Subscribing to url open events") });
    if let Ok(handle) = App::add_app_url_open_listener(|x| redirect_to_url(x.url)).await {
        handle.leak();
    }    
}

#[cfg(any(feature = "android", feature = "ios"))]
fn redirect_to_url(url: String) {    
    use web_sys::window;

    //use crate::state::logging::LoggableEvent;
    // LoggableEvent::try_log(LoggableEvent::Internal { message: format!("Redirect to url: {url}") });
    let Some(url)  = web_sys::Url::new(&url).ok() else {
        return ;};
    let Some(window) = window() else{return ;};

    let Ok(protocol) = window.location().protocol()else {
        return ;};
    let Ok(host) = window.location().host()else {

        return ;};

    url.set_protocol(&protocol);
    url.set_host(&host);
    let url = url.href();
    let Ok(current_href) = window.location().href() else {return ;};

    if current_href != url {
        let _ = window.location().set_href(&url);
        //let _ = window.history().unwrap().push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url));
    }
}
