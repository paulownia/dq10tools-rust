use chrono::{Local, DateTime, TimeZone};
use serde_json::{json, Value};

pub fn get_levels(now: DateTime<Local>) -> Option<Value> {
    let base_point = Local.ymd(2022, 5, 11).and_hms(6, 0, 0);

    if base_point > now {
        return None;
    }

    let d = (now - base_point).num_days();

    let d0 = d % 2 + 1;

    let json = json!({
        "厭悪のルベランギス": d0
    });

    return Some(json)
}


#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};

    #[test]
    fn test_get_levels() {
        let opt = super::get_levels(Local.ymd(2022, 5, 11).and_hms(12, 0, 0));
        assert!(opt.is_some());
        let level = opt.unwrap();
        assert_eq!(level.get("厭悪のルベランギス").unwrap(), 1);

        let opt = super::get_levels(Local.ymd(2022, 5, 13).and_hms(5, 0, 0));
        assert!(opt.is_some());
        let level = opt.unwrap();
        assert_eq!(level.get("厭悪のルベランギス").unwrap(), 2);
    }
}
