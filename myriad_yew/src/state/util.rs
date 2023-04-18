use web_sys::window;

pub fn get_referrer() -> Option<String> {
    let window = window()?;
    let document = window.document()?;
    let referrer = document.referrer();
    if referrer.is_empty() {
        return None;
    }
    Some(referrer)
}
