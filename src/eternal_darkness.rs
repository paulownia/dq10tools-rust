use chrono::{Local, DateTime, TimeZone};
use serde_json::{json, Value};

pub fn get_levels(now: DateTime<Local>) -> Option<Value> {
    let base_point = Local.ymd(2016, 7, 6).and_hms(6, 0, 0);

    if base_point > now {
        return None;
    }

    let d = (now - base_point).num_days();

    let json = json!({
        "レグナード": (d + 2) % 4 + 1,
        "ダークキング": d % 4 + 1,
        "メイヴ": (d + 3) % 4 + 1
    });

    Some(json)
}

#[test]
fn test_get_enemy_level() {
    let e = get_levels(Local.ymd(2019, 7, 7).and_hms(0, 40, 0)).unwrap();
    assert_eq!(e.get("レグナード").unwrap(), 2);
    assert_eq!(e.get("ダークキング").unwrap(), 4);
    assert_eq!(e.get("メイヴ").unwrap(), 3);
}
