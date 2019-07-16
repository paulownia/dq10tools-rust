use super::troop::*;
use chrono::{DateTime, Local, Duration, Timelike, Datelike, TimeZone};

pub struct Event {
    pub started_at: DateTime<Local>,
    pub troop: &'static Troop
}

pub fn get_current_schedule() -> Vec<Event> {
    get_schedule(Local::now())
}

pub fn get_schedule(dt: DateTime<Local>) -> Vec<Event> {
    let mut vec: Vec<Event> = Vec::with_capacity(24);

    let period = calc_period(dt);

    let dt = Local.ymd(dt.year(), dt.month(), dt.day()).and_hms(dt.hour(), 0, 0);

    let troop = get_troop_by_period(period);

    vec.push(Event{
        started_at: dt,
        troop: troop
    });

    for i in 1..24 {
        let troop = get_troop_by_period(period + i);
        let prev_troop = get_troop_by_period(period + i - 1);

        if troop == prev_troop {
            continue;
        }

        let duration = Duration::hours(i as i64);

        vec.push(Event{
            started_at: dt + duration,
            troop: troop
        })
    }

    vec
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    #[test]
    fn test_get_schedule() {
        let dt = chrono::Local.ymd(2018, 9, 22).and_hms(23, 45, 10);
        let schedule = super::get_schedule(dt);
        assert_eq!(schedule.len(), 24);
        assert_eq!(schedule[0].started_at.year(), 2018);
        assert_eq!(schedule[0].started_at.month(), 9);
        assert_eq!(schedule[0].started_at.day(), 22);
        assert_eq!(schedule[0].started_at.hour(), 23);
        assert_eq!(schedule[0].troop.name(), "闇朱の獣牙兵団");
    }
}

