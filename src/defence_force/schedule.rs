use super::troop::*;
use chrono::{DateTime, Datelike, Duration, Local, TimeZone, Timelike, Utc};

pub struct Event {
    pub started_at: DateTime<Local>,
    pub troop: &'static dyn Troop
}

pub fn get_current_schedule() -> Option<Vec<Event>> {
    get_schedule_in(Local::now(), 24)
}

pub fn get_current_schedule_in(count: usize) -> Option<Vec<Event>> {
    get_schedule_in(Local::now(), count)
}

pub fn get_schedule<Tz: TimeZone>(dt: DateTime<Tz>) -> Option<Vec<Event>> {
    get_schedule_in(dt, 24)
}

/// 指定された日時を起点として、防衛軍イベントを24時間分取得する
/// epoch_millis: i64 - Unix epochからの経過時間(ミリ秒)
pub fn get_schedule_from_epoch_millis(epoch_millis:i64) -> Option<Vec<Event>> {
    let result = Utc.timestamp_millis_opt(epoch_millis);
    let dt = result.single()?;
    get_schedule(dt)
}

pub fn get_schedule_in<Tz: TimeZone>(dt: DateTime<Tz>, count: usize) -> Option<Vec<Event>> {
    calc_period(&dt).ok().and_then( |period| {
        let utc = dt.naive_utc();

        Utc.with_ymd_and_hms(utc.year(), utc.month(), utc.day(), utc.hour(), 0, 0).single().map( |started_at| {
            let mut vec: Vec<Event> = Vec::with_capacity(24);

            let troop = get_troop_by_period(period);

            vec.push(Event{
                started_at: started_at.with_timezone(&Local),
                troop: troop
            });

            for i in 1..count {
                let troop = get_troop_by_period(period + i);
                let prev_troop = get_troop_by_period(period + i - 1);

                if troop == prev_troop {
                    continue;
                }

                match Duration::try_hours(i as i64) {
                    Some(duration) => {
                        vec.push(Event{
                            started_at: (started_at + duration).with_timezone(&Local),
                            troop: troop
                        })
                    },
                    None => break
                }
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

    /// 周期の一番最初のスケジュールを確認するテスト
    fn assert_first_time<Tz: TimeZone>(dt: chrono::DateTime<Tz>) {
        let schedule = super::get_schedule(dt).unwrap();
        assert_eq!(schedule.len(), 24);
        assert_eq!(schedule[0].troop.name(), "鋼塊の重滅兵団");
        assert_eq!(schedule[1].troop.name(), "金神の遺宝兵団");

        let jst = schedule[0].started_at.with_timezone(&chrono_tz::Asia::Tokyo);
        assert_eq!(jst.year(), 2025);
        assert_eq!(jst.month(), 5);
        assert_eq!(jst.day(), 14);
        assert_eq!(jst.hour(), 6);
    }

    #[test]
    fn test_get_schedule_east0900() {
        let dt = chrono::FixedOffset::east_opt(9 * 3600).unwrap().with_ymd_and_hms(2025, 5, 14, 6, 0, 0).single().unwrap();
        assert_first_time(dt);
    }
    #[test]
    fn test_get_schedule_jst() {
        let dt = chrono_tz::Asia::Tokyo.with_ymd_and_hms(2025, 5, 14, 6, 10, 34).single().unwrap();
        assert_first_time(dt);
    }
    #[test]
    fn test_get_schedule_utc() {
        let dt = chrono::Utc.with_ymd_and_hms(2025, 5, 13, 21, 10, 34).single().unwrap();
        assert_first_time(dt);
    }
    #[test]
    fn test_get_schedule_sst() {
        let dt = chrono_tz::Asia::Singapore.with_ymd_and_hms(2025, 5, 14, 5, 10, 34).single().unwrap();
        assert_first_time(dt);
    }
    #[test]
    fn text_get_schedule_by_epoch() {
        // 2025-05-18T15:15:34+09:00  // slime -> wood
        let epoch = 1747548934000;
        let schedule = super::get_schedule_from_epoch_millis(epoch).unwrap();
        assert_eq!(schedule[0].troop.name(), "彩虹の粘塊兵団");
        assert_eq!(schedule[1].troop.name(), "腐緑の樹葬兵団");
    }
}

