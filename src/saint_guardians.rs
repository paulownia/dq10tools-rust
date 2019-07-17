use chrono::{Local, DateTime, TimeZone};
use serde_json::{json, Value};

pub fn get_levels(now: DateTime<Local>) -> Option<Value> {
    let base_point = Local.ymd(2018, 4, 20).and_hms(6, 0, 0);

    if base_point > now {
        return None;
    }

    let d = (now - base_point).num_days();

    let d0 = d % 3 + 1;
    let d2 = (d + 2) % 3 + 1;
    let d1 = (d + 1) % 3 + 1;

    let json = json!({
        "レギルラッゾたち": d0,
        "スコルパイド": d2,
        "ジェルザーク": d1
    });

    Some(json)
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    #[test]
    fn test_get_level() {
        let e = super::get_levels(Local.ymd(2019, 7, 5).and_hms(12, 0, 0)).unwrap();
        assert_eq!(e.get("レギルラッゾたち").unwrap(), 1);
        assert_eq!(e.get("スコルパイド").unwrap(), 3);
        assert_eq!(e.get("ジェルザーク").unwrap(), 2);
    }
}
