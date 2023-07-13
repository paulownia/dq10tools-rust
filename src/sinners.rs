use chrono::{Local, DateTime, TimeZone};
use serde_json::{json, Value};

pub fn get_levels(now: DateTime<Local>) -> Option<Value> {
    let base_point = Local.with_ymd_and_hms(2022, 5, 11, 6, 0, 0);

    base_point.single().and_then(|base_point| {
        if base_point > now {
            return None;
        }

        let d = (now - base_point).num_days();

        let d0 = d % 2 + 1;
        let d1 = (d + 1) % 2 + 1;

        let json = json!({
            "厭悪のルベランギス": d0,
            "絶念のアウルモッド": d1,
            "狂禍のフラウソン": d0
        });

        return Some(json)
    })
}


#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};

    #[test]
    fn test_get_levels() {
        let opt = super::get_levels(Local.with_ymd_and_hms(2022, 5, 11, 12, 0, 0).single().unwrap());
        assert!(opt.is_some());
        let level = opt.unwrap();
        assert_eq!(level.get("厭悪のルベランギス").unwrap(), 1);

        let opt = super::get_levels(Local.with_ymd_and_hms(2022, 5, 13, 5, 0, 0).single().unwrap());
        assert!(opt.is_some());
        let level = opt.unwrap();
        assert_eq!(level.get("厭悪のルベランギス").unwrap(), 2);
    }
}
