use chrono::{DateTime, NaiveTime, NaiveDateTime, Datelike, TimeZone, Timelike, Utc, FixedOffset};

pub struct Next {
    pub state: char,
    pub after_minutes: u32,
}

pub fn now() -> NaiveTime {
    from_datetime(Utc::now()).unwrap()
}


pub fn from_timestamp(sec_from_epoch: i64) -> Option<NaiveTime> {
    NaiveDateTime::from_timestamp_opt(sec_from_epoch, 0).and_then(|u| from_naive_utc(u))
}


pub fn from_datetime<Tz: TimeZone>(dt: DateTime<Tz>) -> Option<NaiveTime> {
    from_naive_utc(dt.naive_utc())
}


fn from_naive_utc(nt: NaiveDateTime) -> Option<NaiveTime> {
    FixedOffset::east_opt(9 * 3600).and_then(|offset| {
        let jst = offset.from_utc_datetime(&nt);
        offset.with_ymd_and_hms(jst.year(), jst.month(), jst.day(), 0, 0, 0).single().map(|jst0h| {
            let duration = jst - jst0h;
            let ast = jst0h + duration * 20;
            ast.time()
        })
    })
}

pub fn calc_minutes_to_next(t: NaiveTime) -> Next {
    let h = t.hour();
    let next = if 6 <= h && h < 18 { '夜' } else { '朝' };

    let dh = (29 - h) % 12 + 1;
    let minutes = ((dh * 3600 - t.minute() * 60 - t.second()) as f64 / 1200.0).round() as u32;

    Next {
        state: next,
        after_minutes: minutes,
    }
}


#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use chrono_tz;
    #[test]
    fn test_from_datetime_jst1() {
        let dt = chrono::FixedOffset::east_opt(9 * 3600).unwrap().with_ymd_and_hms(2018, 5, 31, 7, 3, 15).single().unwrap();
        let ast = super::from_datetime(dt).unwrap();
        assert_eq!(ast.hour(), 21);
        assert_eq!(ast.minute(), 5);
    }
    #[test]
    fn test_from_datetime_jst2() {
        let dt = chrono::FixedOffset::east_opt(9 * 3600).unwrap().with_ymd_and_hms(2018, 6, 1, 0, 0, 0).single().unwrap();
        let ast = super::from_datetime(dt).unwrap();
        assert_eq!(ast.hour(), 0);
        assert_eq!(ast.minute(), 0);
    }
    #[test]
    fn test_from_datetime_jst3() {
        let dt = chrono::FixedOffset::east_opt(9 * 3600).unwrap().with_ymd_and_hms(2018, 6, 13, 10, 22, 30).single().unwrap();
        let ast = super::from_datetime(dt).unwrap();
        assert_eq!(ast.hour(), 15);
        assert_eq!(ast.minute(), 30);
    }
    #[test]
    fn test_from_datetime_jst4() {
        let jst = chrono::FixedOffset::east_opt(9 * 3600).unwrap().with_ymd_and_hms(2018, 6, 1, 7, 3, 15).single().unwrap();
        let ast = super::from_datetime(jst).unwrap();
        assert_eq!(ast.hour(), 21);
        assert_eq!(ast.minute(), 5);
    }
    #[test]
    fn test_from_datetime_jst5() {
        let jst = chrono_tz::Asia::Tokyo.with_ymd_and_hms(2018, 6, 1, 0, 0, 0).single().unwrap();
        let ast = super::from_datetime(jst).unwrap();
        assert_eq!(ast.hour(), 0);
        assert_eq!(ast.minute(), 0);
    }
    #[test]
    fn test_from_datetime_sst1() {
        let sst = chrono_tz::Asia::Singapore.with_ymd_and_hms(2018, 5, 30, 23, 0, 0).single().unwrap();
        let ast = super::from_datetime(sst).unwrap();
        assert_eq!(ast.hour(), 0);
        assert_eq!(ast.minute(), 0);
    }
    #[test]
    fn test_from_datetime_utc1() {
        let utc = chrono::Utc.with_ymd_and_hms(2018, 6, 13, 1, 22, 30).single().unwrap();
        let ast = super::from_datetime(utc).unwrap();
        assert_eq!(ast.hour(), 15);
        assert_eq!(ast.minute(), 30);
    }
    #[test]
    fn test_calc_minutes_to_next0() {
        let t = chrono::NaiveTime::from_hms_milli_opt(6, 0, 1, 0).unwrap();
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '夜');
        assert_eq!(s.after_minutes, 36);
    }
    #[test]
    fn test_calc_minutes_to_next1() {
        let t = chrono::NaiveTime::from_hms_milli_opt(12, 45, 30, 0).unwrap();
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '夜');
        assert_eq!(s.after_minutes, 16);
    }
    #[test]
    fn test_calc_minutes_to_next2() {
        let t = chrono::NaiveTime::from_hms_milli_opt(5, 45, 30, 1).unwrap();
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '朝');
        assert_eq!(s.after_minutes, 1);
    }
    #[test]
    fn test_calc_minutes_to_next3() {
        let t = chrono::NaiveTime::from_hms_milli_opt(23, 45, 30, 1).unwrap();
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '朝');
        assert_eq!(s.after_minutes, 19);
    }
    #[test]
    fn test_calc_minutes_to_next4() {
        let t = chrono::NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '朝');
        assert_eq!(s.after_minutes, 18);
    }
    #[test]
    fn test_calc_minutes_to_next5() {
        let t = chrono::NaiveTime::from_hms_milli_opt(6, 0, 0, 0).unwrap();
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '夜');
        assert_eq!(s.after_minutes, 36);
    }
    #[test]
    fn test_calc_minutes_to_next6() {
        let t = chrono::NaiveTime::from_hms_milli_opt(18, 0, 0, 0).unwrap();
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '朝');
        assert_eq!(s.after_minutes, 36);
    }
    #[test]
    fn test_from_timestamp() {
        let epoch:i64 = 1656735654; // 2022/07/02 13:20:54
        let ast = super::from_timestamp(epoch).unwrap();
        assert_eq!(ast.hour(), 2);
        assert_eq!(ast.minute(), 58);
    }
    #[test]
    fn test_same_from_timestamp_and_utc_now() {
        let utc = chrono::Utc::now();
        // ミリ秒を含めない含めない時刻に変換
        let utc = chrono::Utc.with_ymd_and_hms(utc.year(), utc.month(), utc.day(), utc.hour(), utc.minute(), utc.second()).single().unwrap();
        let epoch = utc.timestamp();
        let ast1 = super::from_timestamp(epoch).unwrap();
        let ast2 = super::from_datetime(utc).unwrap();
        assert_eq!(ast1, ast2)
    }
    #[test]
    fn test_same_from_timestamp_and_local_now() {
        let l = chrono::Local::now();
        // ミリ秒を含めない含めない時刻に変換
        let l = chrono::Local.with_ymd_and_hms(l.year(), l.month(), l.day(), l.hour(), l.minute(), l.second()).single().unwrap();
        let e = l.timestamp();
        let ast1 = super::from_timestamp(e).unwrap();
        let ast2 = super::from_datetime(l).unwrap();
        assert_eq!(ast1, ast2)
    }
    #[test]
    fn test_same_utf_now_and_local_now() {
        let l = chrono::Local::now();
        let l = chrono::Local.with_ymd_and_hms(l.year(), l.month(), l.day(), l.hour(), l.minute(), l.second()).single().unwrap();
        let u = chrono::Utc::now();
        let u = chrono::Utc.with_ymd_and_hms(u.year(), u.month(), u.day(), u.hour(), u.minute(), u.second()).single().unwrap();
        let ast1 = super::from_datetime(l).unwrap();
        let ast2 = super::from_datetime(u).unwrap();
        assert_eq!(ast1, ast2)
    }
}
