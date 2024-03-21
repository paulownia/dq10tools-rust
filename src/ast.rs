use chrono::{DateTime, NaiveTime, NaiveDateTime, Datelike, TimeZone, Timelike, Utc, FixedOffset};

#[derive(PartialEq, Eq, Debug)]
pub struct AST(NaiveTime);

impl AST {
    pub fn state(&self) -> State {
        let h = self.0.hour();
        if 6 <= h && h < 18 {
            State::Day
        } else {
            State::Night
        }
    }
    pub fn state_change_in(&self) -> u32 {
        let t = self.0;
        let h = self.0.hour();
        let dh = (29 - h) % 12 + 1;
        ((dh * 3600 - t.minute() * 60 - t.second()) as f64 / 1200.0).round() as u32
    }
    pub fn time(&self) -> NaiveTime {
        self.0
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum State {
    Day,
    Night,
}

impl State {
    pub fn change(&self) -> State {
        match self {
            State::Day => State::Night,
            State::Night => State::Day,
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Day => write!(f, "{}", "朝"),
            State::Night => write!(f, "{}", "夜"),
        }
   }
}

pub fn now() -> AST {
    from_datetime(Utc::now()).unwrap()
}


pub fn from_timestamp(sec_from_epoch: i64) -> Option<AST> {
    DateTime::from_timestamp(sec_from_epoch, 0).and_then(|u| from_datetime(u))
}

pub fn from_timestamp_millis(millis_from_epoch: i64) -> Option<AST> {
    DateTime::from_timestamp_millis(millis_from_epoch).and_then(|u| from_datetime(u))
}

pub fn from_datetime<Tz: TimeZone>(dt: DateTime<Tz>) -> Option<AST> {
    from_naive_utc(dt.naive_utc())
}


fn from_naive_utc(nt: NaiveDateTime) -> Option<AST> {
    FixedOffset::east_opt(9 * 3600).and_then(|offset| {
        let jst = offset.from_utc_datetime(&nt);
        offset.with_ymd_and_hms(jst.year(), jst.month(), jst.day(), 0, 0, 0).single().map(|jst0h| {
            let duration = jst - jst0h;
            let ast = jst0h + duration * 20;
            ast.time()
        })
    }).map( |naive_time| AST(naive_time) )
}


#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use chrono_tz;
    #[test]
    fn test_from_datetime_jst1() {
        let dt = chrono::FixedOffset::east_opt(9 * 3600).unwrap().with_ymd_and_hms(2018, 5, 31, 7, 3, 15).single().unwrap();
        let ast = super::from_datetime(dt).unwrap();
        assert_eq!(ast.time().hour(), 21);
        assert_eq!(ast.time().minute(), 5);
    }
    #[test]
    fn test_from_datetime_jst2() {
        let dt = chrono::FixedOffset::east_opt(9 * 3600).unwrap().with_ymd_and_hms(2018, 6, 1, 0, 0, 0).single().unwrap();
        let ast = super::from_datetime(dt).unwrap();
        assert_eq!(ast.time().hour(), 0);
        assert_eq!(ast.time().minute(), 0);
    }
    #[test]
    fn test_from_datetime_jst3() {
        let dt = chrono::FixedOffset::east_opt(9 * 3600).unwrap().with_ymd_and_hms(2018, 6, 13, 10, 22, 30).single().unwrap();
        let ast = super::from_datetime(dt).unwrap();
        assert_eq!(ast.time().hour(), 15);
        assert_eq!(ast.time().minute(), 30);
    }
    #[test]
    fn test_from_datetime_jst4() {
        let jst = chrono::FixedOffset::east_opt(9 * 3600).unwrap().with_ymd_and_hms(2018, 6, 1, 7, 3, 15).single().unwrap();
        let ast = super::from_datetime(jst).unwrap();
        assert_eq!(ast.time().hour(), 21);
        assert_eq!(ast.time().minute(), 5);
    }
    #[test]
    fn test_from_datetime_jst5() {
        let jst = chrono_tz::Asia::Tokyo.with_ymd_and_hms(2018, 6, 1, 0, 0, 0).single().unwrap();
        let ast = super::from_datetime(jst).unwrap();
        assert_eq!(ast.time().hour(), 0);
        assert_eq!(ast.time().minute(), 0);
    }
    #[test]
    fn test_from_datetime_sst1() {
        let sst = chrono_tz::Asia::Singapore.with_ymd_and_hms(2018, 5, 30, 23, 0, 0).single().unwrap();
        let ast = super::from_datetime(sst).unwrap();
        assert_eq!(ast.time().hour(), 0);
        assert_eq!(ast.time().minute(), 0);
    }
    #[test]
    fn test_from_datetime_utc1() {
        let utc = chrono::Utc.with_ymd_and_hms(2018, 6, 13, 1, 22, 30).single().unwrap();
        let ast = super::from_datetime(utc).unwrap();
        assert_eq!(ast.time().hour(), 15);
        assert_eq!(ast.time().minute(), 30);
    }
    #[test]
    fn test_calc_minutes_to_next0() {
        let t = chrono::NaiveTime::from_hms_milli_opt(6, 0, 1, 0).unwrap();
        let s = super::AST(t);
        assert_eq!(s.state().change().to_string(), "夜");
        assert_eq!(s.state_change_in(), 36);
    }
    #[test]
    fn test_calc_minutes_to_next1() {
        let t = chrono::NaiveTime::from_hms_milli_opt(12, 45, 30, 0).unwrap();
        let s = super::AST(t);
        assert_eq!(s.state().change().to_string(), "夜");
        assert_eq!(s.state_change_in(), 16);
    }
    #[test]
    fn test_calc_minutes_to_next2() {
        let t = chrono::NaiveTime::from_hms_milli_opt(5, 45, 30, 1).unwrap();
        let s = super::AST(t);
        assert_eq!(s.state().change().to_string(), "朝");
        assert_eq!(s.state_change_in(), 1);
    }
    #[test]
    fn test_calc_minutes_to_next3() {
        let t = chrono::NaiveTime::from_hms_milli_opt(23, 45, 30, 1).unwrap();
        let s = super::AST(t);
        assert_eq!(s.state().change().to_string(), "朝");
        assert_eq!(s.state_change_in(), 19);
    }
    #[test]
    fn test_calc_minutes_to_next4() {
        let t = chrono::NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
        let s = super::AST(t);
        assert_eq!(s.state().change().to_string(), "朝");
        assert_eq!(s.state_change_in(), 18);
    }
    #[test]
    fn test_calc_minutes_to_next5() {
        let t = chrono::NaiveTime::from_hms_milli_opt(6, 0, 0, 0).unwrap();
        let s = super::AST(t);
        assert_eq!(s.state().change().to_string(), "夜");
        assert_eq!(s.state_change_in(), 36);
    }
    #[test]
    fn test_calc_minutes_to_next6() {
        let t = chrono::NaiveTime::from_hms_milli_opt(18, 0, 0, 0).unwrap();
        let s = super::AST(t);
        assert_eq!(s.state().change().to_string(), "朝");
        assert_eq!(s.state_change_in(), 36);
    }
    #[test]
    fn test_from_timestamp() {
        let epoch:i64 = 1656735654; // 2022/07/02 13:20:54
        let ast = super::from_timestamp(epoch).unwrap();
        assert_eq!(ast.time().hour(), 2);
        assert_eq!(ast.time().minute(), 58);
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
    fn test_same_utc_now_and_local_now() {
        let l = chrono::Local::now();
        let l = chrono::Local.with_ymd_and_hms(l.year(), l.month(), l.day(), l.hour(), l.minute(), l.second()).single().unwrap();
        let u = chrono::Utc::now();
        let u = chrono::Utc.with_ymd_and_hms(u.year(), u.month(), u.day(), u.hour(), u.minute(), u.second()).single().unwrap();
        let ast1 = super::from_datetime(l).unwrap();
        let ast2 = super::from_datetime(u).unwrap();
        assert_eq!(ast1, ast2)
    }
    #[test]
    fn test_same_from_timestamp_and_timestamp_millis() {
        let ast1 = super::from_timestamp_millis(1678671352000).unwrap();
        let ast2 = super::from_timestamp(1678671352).unwrap();
        assert_eq!(ast1, ast2)
    }
}
