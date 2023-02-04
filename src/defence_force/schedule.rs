use super::troop::*;
use chrono::{DateTime, Local, Utc, Duration, Timelike, Datelike, TimeZone};

pub struct Event {
    pub started_at: DateTime<Local>,
    pub troop: &'static dyn Troop
}

pub fn get_current_schedule() -> Option<Vec<Event>> {
    get_schedule(Local::now())
}

pub fn get_schedule<Tz: TimeZone>(dt: DateTime<Tz>) -> Option<Vec<Event>> {
    calc_period(&dt).ok().and_then( |period| {
        let utc = dt.naive_utc();

        Utc.with_ymd_and_hms(utc.year(), utc.month(), utc.day(), utc.hour(), 0, 0).single().map( |started_at| {
            let mut vec: Vec<Event> = Vec::with_capacity(24);

            let troop = get_troop_by_period(period);

            vec.push(Event{
                started_at: started_at.with_timezone(&Local),
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
                    started_at: (started_at + duration).with_timezone(&Local),
                    troop: troop
                })
            }

            vec
        })
    })
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use chrono_tz;
    #[test]
    fn test_get_schedule_none_before_basepoint() {
        let dt = chrono::Local.with_ymd_and_hms(2018, 9, 22, 23, 45, 10).single().unwrap();
        let schedule = super::get_schedule(dt);
        assert!(schedule.is_none());
    }
    #[test]
    fn test_get_schedule_east0900() {
        let dt = chrono::FixedOffset::east_opt(9 * 3600).unwrap().with_ymd_and_hms(2023, 2, 5, 14, 45, 10).single().unwrap();
        let schedule = super::get_schedule(dt).unwrap();
        assert_eq!(schedule.len(), 24);
        assert_eq!(schedule[0].troop.name(), "灰塵の竜鱗兵団");
        assert_eq!(schedule[1].troop.name(), "全兵団");

        let jst = schedule[0].started_at.with_timezone(&chrono_tz::Asia::Tokyo);
        assert_eq!(jst.year(), 2023);
        assert_eq!(jst.month(), 2);
        assert_eq!(jst.day(), 5);
        assert_eq!(jst.hour(), 14);
    }
    #[test]
    fn test_get_schedule_jst() {
        let dt = chrono_tz::Asia::Tokyo.with_ymd_and_hms(2023, 2, 7, 12, 10, 34).single().unwrap();
        let schedule = super::get_schedule(dt).unwrap();
        assert_eq!(schedule.len(), 24);
        assert_eq!(schedule[0].troop.name(), "灰塵の竜鱗兵団");
        assert_eq!(schedule[1].troop.name(), "腐緑の樹葬兵団");

        let jst = schedule[0].started_at.with_timezone(&chrono_tz::Asia::Tokyo);
        assert_eq!(jst.year(), 2023);
        assert_eq!(jst.month(), 2);
        assert_eq!(jst.day(), 7);
        assert_eq!(jst.hour(), 12);
    }
    #[test]
    fn test_get_schedule_utc() {
        let dt = chrono::Utc.with_ymd_and_hms(2023, 2, 7, 3, 10, 34).single().unwrap();
        let schedule = super::get_schedule(dt).unwrap();
        assert_eq!(schedule.len(), 24);
        assert_eq!(schedule[0].troop.name(), "灰塵の竜鱗兵団");
        assert_eq!(schedule[1].troop.name(), "腐緑の樹葬兵団");

        let jst = schedule[0].started_at.with_timezone(&chrono_tz::Asia::Tokyo);
        assert_eq!(jst.year(), 2023);
        assert_eq!(jst.month(), 2);
        assert_eq!(jst.day(), 7);
        assert_eq!(jst.hour(), 12);
    }
    #[test]
    fn test_get_schedule_sst() {
        let dt = chrono_tz::Asia::Singapore.with_ymd_and_hms(2023, 2, 7, 11, 10, 34).single().unwrap();
        let schedule = super::get_schedule(dt).unwrap();
        assert_eq!(schedule.len(), 24);
        assert_eq!(schedule[0].troop.name(), "灰塵の竜鱗兵団");
        assert_eq!(schedule[1].troop.name(), "腐緑の樹葬兵団");

        let jst = schedule[0].started_at.with_timezone(&chrono_tz::Asia::Tokyo);
        assert_eq!(jst.year(), 2023);
        assert_eq!(jst.month(), 2);
        assert_eq!(jst.day(), 7);
        assert_eq!(jst.hour(), 12);
    }
}

