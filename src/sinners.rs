use chrono::{DateTime, Days, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::Asia::Tokyo;
use serde_json::{json, Value};

fn get_base_point() -> NaiveDateTime {
    Tokyo.with_ymd_and_hms(2022, 5, 11, 6, 0, 0).unwrap().naive_utc()
}

fn to_term<Tz: TimeZone>(now: NaiveDateTime, tz: Tz) -> (DateTime<Tz>, DateTime<Tz>) {
    let boundary_time = NaiveTime::from_hms_opt(21, 0, 0).unwrap();
    let now_time = now.time();

    let delta_one_day = Days::new(1);
    let (from, to) = if now_time > boundary_time {
        let from = now.date().and_hms_opt(21, 0, 0).unwrap();
        let to  = now.date().and_hms_opt(20, 59, 59).unwrap() + delta_one_day;
        (from, to)
    } else {
        let from = now.date().and_hms_opt(21, 0, 0).unwrap() - delta_one_day;
        let to  = now.date().and_hms_opt(20, 59, 59).unwrap();
        (from, to)
    };

    let from_tz = tz.from_utc_datetime(&from);
    let to_tz = tz.from_utc_datetime(&to);

    (from_tz, to_tz)
}

pub fn get_current_levels() -> Option<Value> {
    get_levels(Utc::now(), Local)
}

pub fn get_levels<Tz: TimeZone>(now: DateTime<Utc>, tz: Tz) -> Option<Value> {
    let base_point = get_base_point();
    let given_point = now.naive_utc();

    if given_point < base_point {
        return None;
    }

    let (from, to) = to_term(given_point, tz);

    let d = (given_point - base_point).num_days();
    let d0 = d % 2 + 1;
    let d1 = (d + 1) % 2 + 1;

    let json = json!({
        "level": {
            "絶念のアウルモッド": d1,
            "狂禍のフラウソン": d0,
            "悲愴のウィリーデ" : d1,
            "燦滅のノクゼリア": d0
        },
        "term": {
            "from": from.to_rfc3339(),
            "to": to.to_rfc3339()
        }
    });

    Some(json)
}


#[cfg(test)]
mod tests {
    use chrono::{TimeZone, FixedOffset, Utc};
    use chrono_tz::Asia::{Tokyo, Singapore};

    #[test]
    fn test_get_levels() {
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();

        let dt = jst.with_ymd_and_hms(2022, 5, 11, 12, 0, 0).single().unwrap();
        let opt = super::get_levels(dt.to_utc(), jst);
        assert!(opt.is_some());
        let data = opt.unwrap();
        assert_eq!(data["level"]["燦滅のノクゼリア"], 1);
        assert_eq!(data["term"]["from"].as_str(), Some("2022-05-11T06:00:00+09:00"));
        assert_eq!(data["term"]["to"].as_str(), Some("2022-05-12T05:59:59+09:00"));

        let dt = jst.with_ymd_and_hms(2022, 5, 13, 5, 0, 0).single().unwrap();
        let opt = super::get_levels(dt.to_utc(), jst);
        assert!(opt.is_some());
        let data = opt.unwrap();
        assert_eq!(data["level"]["燦滅のノクゼリア"], 2);
        assert_eq!(data["term"]["from"].as_str(), Some("2022-05-12T06:00:00+09:00"));
        assert_eq!(data["term"]["to"].as_str(), Some("2022-05-13T05:59:59+09:00"));

        let dt = Tokyo.with_ymd_and_hms(2022, 5, 13, 5, 0, 0).single().unwrap();
        let opt = super::get_levels(dt.to_utc(), Tokyo);
        assert!(opt.is_some());
        let data = opt.unwrap();
        assert_eq!(data["level"]["燦滅のノクゼリア"], 2);
        assert_eq!(data["term"]["from"].as_str(), Some("2022-05-12T06:00:00+09:00"));
        assert_eq!(data["term"]["to"].as_str(), Some("2022-05-13T05:59:59+09:00"));
    }

    #[test]
    fn test_get_levels_utc() {
        // UTCの2022/5/11 12:00:00はJSTの2022/5/11 21:00:00
        let dt = Utc.with_ymd_and_hms(2022, 5, 11, 12, 0, 0).single().unwrap();
        let opt = super::get_levels(dt, Utc);
        assert!(opt.is_some());
        let data = opt.unwrap();
        assert_eq!(data["level"]["燦滅のノクゼリア"], 1);
        assert_eq!(data["term"]["from"], "2022-05-10T21:00:00+00:00");
        assert_eq!(data["term"]["to"], "2022-05-11T20:59:59+00:00");

        // UTCの2022/5/13 5:00:00はJSTの2022/5/13 14:00:00
        let dt = Utc.with_ymd_and_hms(2022, 5, 13, 5, 0, 0).single().unwrap();
        let opt = super::get_levels(dt, Utc);
        assert!(opt.is_some());
        let data = opt.unwrap();
        assert_eq!(data["level"]["燦滅のノクゼリア"], 1);
        assert_eq!(data["term"]["from"], "2022-05-12T21:00:00+00:00");
        assert_eq!(data["term"]["to"], "2022-05-13T20:59:59+00:00");
    }

    #[test]
    fn test_get_levels_sst() {
        // UTCの2022/5/11 12:00:00はSSTの2022/5/11 20:00:00
        let dt = Utc.with_ymd_and_hms(2022, 5, 11, 12, 0, 0).single().unwrap();
        let opt = super::get_levels(dt, Singapore);
        assert!(opt.is_some());
        let data = opt.unwrap();
        assert_eq!(data["level"]["燦滅のノクゼリア"], 1);
        assert_eq!(data["term"]["from"].as_str(), Some("2022-05-11T05:00:00+08:00"));
        assert_eq!(data["term"]["to"].as_str(), Some("2022-05-12T04:59:59+08:00"));
    }
}
