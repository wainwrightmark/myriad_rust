use serde::*;
#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum GameTiming {
    Started { utc_time_milliseconds: i64 },
    Finished { total_milliseconds: u64 },
    Unknown,
}

impl Default for GameTiming {
    fn default() -> Self {
        let js_today = js_sys::Date::new_0();
        let utc_time = js_today.get_time();
        let utc_time_milliseconds = utc_time.floor() as i64;
        Self::Started {
            utc_time_milliseconds,
        }
    }
}
