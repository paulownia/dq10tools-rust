use chrono::{DateTime, NaiveTime, Datelike, TimeZone, Timelike, Utc, FixedOffset, Local};

pub struct Next {
    pub state: char,
    pub after_minutes: u32,
}

pub fn now() -> NaiveTime {
    from_utc(Utc::now())
}

pub fn from_epoch(epoch: i64) -> NaiveTime {
    from_utc(Utc.timestamp(epoch, 0))
}

pub fn from_utc(utc: DateTime<Utc>) ->NaiveTime {
    let offset = FixedOffset::east(9 * 3600);
    let jst = DateTime::<FixedOffset>::from_utc(utc.naive_utc(), offset);
    let jst0h = offset.ymd(jst.year(), jst.month(), jst.day()).and_hms(0, 0, 0);

    let duration = jst - jst0h;
    let ast = jst0h + duration * 20;
    ast.time()
}

#[deprecated(since="6.1.2", note="please use `from_utc` instead")]
pub fn convert(jst: DateTime<Local>) -> NaiveTime {
    let jst0h = Local.ymd(jst.year(), jst.month(), jst.day()).and_hms(0, 0, 0);
    let duration = jst - jst0h;
    let ast = jst0h + duration * 20;
    ast.time()
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
    #[test]
    fn test_convert1() {
        let dt = chrono::Local.ymd(2018, 5, 31).and_hms(7, 3, 15);
        let ast = super::convert(dt);
        assert_eq!(ast.hour(), 21);
        assert_eq!(ast.minute(), 5);
    }
    #[test]
    fn test_convert2() {
        let dt = chrono::Local.ymd(2018, 6, 1).and_hms(0, 0, 0);
        let ast = super::convert(dt);
        assert_eq!(ast.hour(), 0);
        assert_eq!(ast.minute(), 0);
    }
    #[test]
    fn test_convert3() {
        let dt = chrono::Local.ymd(2018, 6, 13).and_hms(10, 22, 30);
        let ast = super::convert(dt);
        assert_eq!(ast.hour(), 15);
        assert_eq!(ast.minute(), 30);
    }
    #[test]
    fn test_from_utc1() {
        let jst = chrono::FixedOffset::east(9 * 3600).ymd(2018, 6, 1).and_hms(7, 3, 15);
        let utc = chrono::DateTime::<Utc>::from_utc(jst.naive_utc(), Utc);
        let ast = super::from_utc(utc);
        assert_eq!(ast.hour(), 21);
        assert_eq!(ast.minute(), 5);
    }
    #[test]
    fn test_from_utc2() {
        let jst = chrono::FixedOffset::east(9 * 3600).ymd(2018, 6, 1).and_hms(0, 0, 0);
        let utc = chrono::DateTime::<Utc>::from_utc(jst.naive_utc(), Utc);
        let ast = super::from_utc(utc);
        assert_eq!(ast.hour(), 0);
        assert_eq!(ast.minute(), 0);
    }
    #[test]
    fn test_from_utc3() {
        let utc = chrono::Utc.ymd(2018, 6, 13).and_hms(1, 22, 30);
        let ast = super::from_utc(utc);
        assert_eq!(ast.hour(), 15);
        assert_eq!(ast.minute(), 30);
    }
    #[test]
    fn test_calc_minutes_to_next0() {
        let t = chrono::NaiveTime::from_hms_milli(6, 0, 1, 0);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '夜');
        assert_eq!(s.after_minutes, 36);
    }
    #[test]
    fn test_calc_minutes_to_next1() {
        let t = chrono::NaiveTime::from_hms_milli(12, 45, 30, 0);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '夜');
        assert_eq!(s.after_minutes, 16);
    }
    #[test]
    fn test_calc_minutes_to_next2() {
        let t = chrono::NaiveTime::from_hms_milli(5, 45, 30, 1);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '朝');
        assert_eq!(s.after_minutes, 1);
    }
    #[test]
    fn test_calc_minutes_to_next3() {
        let t = chrono::NaiveTime::from_hms_milli(23, 45, 30, 1);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '朝');
        assert_eq!(s.after_minutes, 19);
    }
    #[test]
    fn test_calc_minutes_to_next4() {
        let t = chrono::NaiveTime::from_hms_milli(0, 0, 0, 0);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '朝');
        assert_eq!(s.after_minutes, 18);
    }
    #[test]
    fn test_calc_minutes_to_next5() {
        let t = chrono::NaiveTime::from_hms_milli(6, 0, 0, 0);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '夜');
        assert_eq!(s.after_minutes, 36);
    }
    #[test]
    fn test_calc_minutes_to_next6() {
        let t = chrono::NaiveTime::from_hms_milli(18, 0, 0, 0);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '朝');
        assert_eq!(s.after_minutes, 36);
    }
    #[test]
    fn test_from_epoch() {
        let epoch:i64 = 1656735654; // 2022/07/02 13:20:54
        let ast = super::from_epoch(epoch);
        assert_eq!(ast.hour(), 2);
        assert_eq!(ast.minute(), 58);
    }
    #[test]
    fn test_same_from_epoch_and_now() {
        let utc = chrono::Utc::now();
        // ミリ秒を含めない含めない時刻に変換
        let utc = chrono::Utc.ymd(utc.year(), utc.month(), utc.day()).and_hms(utc.hour(), utc.minute(), utc.second());
        let epoch = utc.timestamp() as i64;
        let ast1 = super::from_epoch(epoch);
        let ast2 = super::from_utc(utc);
        assert_eq!(ast1, ast2)
    }
}
