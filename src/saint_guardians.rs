use chrono::{Local, DateTime, TimeZone};
use serde_json::{json, Value};

pub fn get_levels(now: DateTime<Local>) -> Option<Value> {
    let base_point_opt = Local.with_ymd_and_hms(2018, 4, 20, 6, 0, 0);


    base_point_opt.single().and_then( |base_point| {
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
            "ジェルザーク": d1,
            "ガルドドン": d1,
            "デルメゼ": d2,
            "バラシュナ": d0
        });

        Some(json)
    })
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    #[test]
    fn test_get_level() {
        let e = super::get_levels(Local.with_ymd_and_hms(2019, 7, 5, 12, 0, 0).single().unwrap()).unwrap();
        assert_eq!(e.get("レギルラッゾたち").unwrap(), 1);
        assert_eq!(e.get("スコルパイド").unwrap(), 3);
        assert_eq!(e.get("デルメゼ").unwrap(), 3);
        assert_eq!(e.get("ジェルザーク").unwrap(), 2);
        assert_eq!(e.get("ガルドドン").unwrap(), 2);

        let e = super::get_levels(Local.with_ymd_and_hms(2020, 11, 18, 6, 0, 0).single().unwrap()).unwrap();
        assert_eq!(e.get("レギルラッゾたち").unwrap(), 2);
        assert_eq!(e.get("スコルパイド").unwrap(), 1);
        assert_eq!(e.get("デルメゼ").unwrap(), 1);
        assert_eq!(e.get("ジェルザーク").unwrap(), 3);
        assert_eq!(e.get("ガルドドン").unwrap(), 3);
    }
}
