use js_sys::Date;
use wasm_bindgen::JsValue;

pub fn format_duration(minutes: i32) -> String {
    let minutes = minutes.max(0);
    let minutes_part = minutes % 60;
    let hours_part = minutes / 60;

    match (hours_part, minutes_part) {
        (0, m) => format!("{} m", m),
        (h, 0) => format!("{} h", h),
        (h, m) => format!("{} h {} m", h, m),
    }
}

pub fn format_datetime(offset_minutes: i32) -> String {
    let now_ms = Date::new_0().get_time();
    let future_ms = now_ms + (offset_minutes as f64) * 60.0 * 1000.0;
    let future = Date::new(&JsValue::from(future_ms));

    let hours_24 = future.get_hours() as i32;
    let minutes = future.get_minutes() as i32;

    let (hours_12, ampm) = if hours_24 == 0 {
        (12, "AM")
    } else if hours_24 < 12 {
        (hours_24, "AM")
    } else if hours_24 == 12 {
        (12, "PM")
    } else {
        (hours_24 - 12, "PM")
    };

    format!("{}:{:02} {}", hours_12, minutes, ampm)
}
