use chrono::{Local, DateTime, NaiveTime, Datelike, TimeZone, Timelike};

pub struct Next {
    pub state: char,
    pub after_minutes: u32,
}

pub fn now() -> NaiveTime {
    convert(Local::now())
}

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
    fn test_calc_minutes_to_next0() {
        let t = chrono::NaiveTime::from_hms_milli(6, 0, 1, 0);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '夜');
        assert_eq!(s.after_minutes, 36)
    }
    #[test]
    fn test_calc_minutes_to_next1() {
        let t = chrono::NaiveTime::from_hms_milli(12, 45, 30, 0);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '夜');
        assert_eq!(s.after_minutes, 16)
    }
    #[test]
    fn test_calc_minutes_to_next2() {
        let t = chrono::NaiveTime::from_hms_milli(5, 45, 30, 1);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '朝');
        assert_eq!(s.after_minutes, 1)
    }
    #[test]
    fn test_calc_minutes_to_next3() {
        let t = chrono::NaiveTime::from_hms_milli(23, 45, 30, 1);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '朝');
        assert_eq!(s.after_minutes, 19)
    }
    #[test]
    fn test_calc_minutes_to_next4() {
        let t = chrono::NaiveTime::from_hms_milli(0, 0, 0, 0);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '朝');
        assert_eq!(s.after_minutes, 18)
    }
    #[test]
    fn test_calc_minutes_to_next5() {
        let t = chrono::NaiveTime::from_hms_milli(6, 0, 0, 0);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '夜');
        assert_eq!(s.after_minutes, 36)
    }
    #[test]
    fn test_calc_minutes_to_next6() {
        let t = chrono::NaiveTime::from_hms_milli(18, 0, 0, 0);
        let s = super::calc_minutes_to_next(t);
        assert_eq!(s.state, '朝');
        assert_eq!(s.after_minutes, 36)
    }
}
