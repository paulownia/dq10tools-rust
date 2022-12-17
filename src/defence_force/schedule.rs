use super::troop::*;
use chrono::{DateTime, Local, Duration, Timelike, Datelike, TimeZone};

pub struct Event {
    pub started_at: DateTime<Local>,
    pub troop: &'static dyn Troop
}

pub fn get_current_schedule() -> Option<Vec<Event>> {
    get_schedule(Local::now())
}

pub fn get_schedule(dt: DateTime<Local>) -> Option<Vec<Event>> {
    let mut vec: Vec<Event> = Vec::with_capacity(24);

    let period = calc_period(dt);

    if period.is_err() {
        return None;
    }

    let period = period.unwrap();

    let dt_opt = Local.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), dt.hour(), 0, 0).single();
    dt_opt.map( |dt| {
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
    })
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    #[test]
    fn test_get_schedule() {
        let dt = chrono::Local.with_ymd_and_hms(2018, 9, 22, 23, 45, 10).single().unwrap();
        let schedule = super::get_schedule(dt);
        assert!(schedule.is_none());
    }
    #[test]
    fn test_get_schedule2() {
        let dt = chrono::Local.with_ymd_and_hms(2022, 7, 6, 14, 45, 10).single().unwrap();
        let schedule = super::get_schedule(dt);
        let schedule = schedule.unwrap();
        assert_eq!(schedule.len(), 24);
        assert_eq!(schedule[0].started_at.year(), 2022);
        assert_eq!(schedule[0].started_at.month(), 7);
        assert_eq!(schedule[0].started_at.day(), 6);
        assert_eq!(schedule[0].started_at.hour(), 14);
        assert_eq!(schedule[0].troop.name(), "白雲の冥翼軍団");
    }
}

