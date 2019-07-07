use chrono::{Local, DateTime, NaiveTime, Datelike, TimeZone, Timelike};

pub struct Next {
    pub state: char,
    pub after_minutes: u32,
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
    let minutes = (dh * 3600 - t.minute() * 60 - t.second()) / 1200;

    Next {
        state: next,
        after_minutes: minutes,
    }
}
