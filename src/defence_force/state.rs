use crate::defence_force::troop::*;
use chrono::{DateTime, Local, Duration, Timelike, TimeZone};

pub struct State {
    pub troop: &'static dyn Troop,
    pub next_troop: &'static dyn Troop,
    pub next_in: u32,
    pub changed_at: chrono::DateTime<Local>
}

pub fn get_current_state() -> Option<State> {
    get_state(Local::now())
}

pub fn get_state<Tz: TimeZone>(dt: DateTime<Tz>) -> Option<State> {
    calc_period(&dt).ok().and_then( |period| {
        let mut next_in = 60 - dt.naive_utc().minute();

        let mut next_period = period + 1;

        while is_same_troop(period, next_period) {
            next_period += 1;
            next_in += 60;
        }

        let duration = Duration::try_minutes(next_in as i64)?;

        let changed_at = dt.clone() + duration;

        Some(State {
            troop: get_troop_by_period(period),
            next_troop: get_troop_by_period(next_period),
            next_in: next_in,
            changed_at: changed_at.with_timezone(&Local)
        })
    })
}

fn is_same_troop(p1: usize, p2: usize) -> bool {
    get_troop_by_period(p1) == get_troop_by_period(p2)
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use chrono_tz;

    #[test]
    fn test_get_state_is_none_before_basepoint() {
        let dt = chrono::Local.with_ymd_and_hms(2018, 9, 20, 15, 3, 15).single().unwrap();
        let state = super::get_state(dt);
        assert!(state.is_none());
    }

    #[test]
    fn test_get_state_jst() {
        let dt = chrono_tz::Asia::Tokyo.with_ymd_and_hms(2024, 10, 25, 13, 4, 0).single().unwrap();
        let state = super::get_state(dt);
        let state = state.unwrap();
        assert!(state.troop.name().contains("鉄機"));
        assert!(state.next_troop.name().contains("凶蟲"));
        assert_eq!(state.next_in, 56);
    }
    #[test]
    fn test_get_state_east0900() {
        let dt = chrono::FixedOffset::east_opt(9 * 3600).unwrap().with_ymd_and_hms(2024, 10, 25, 23, 4, 0).single().unwrap();
        let state = super::get_state(dt);
        let state = state.unwrap();
        assert!(state.troop.name().contains("造魔"));
        assert!(state.next_troop.name().contains("海妖"));
        assert_eq!(state.next_in, 56);
    }
}
