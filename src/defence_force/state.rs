use crate::defence_force::troop::*;
use chrono::{DateTime, Local, Duration, Timelike};

pub struct State {
    pub troop: &'static Troop,
    pub next_troop: &'static Troop,
    pub next_in: u32,
    pub changed_at: chrono::DateTime<Local>
}

pub fn get_current_state() -> State {
    get_state(Local::now())
}

pub fn get_state(dt: DateTime<Local>) -> State {
    let period = calc_period(dt);

    let mut next_in = 60 - dt.minute();

    let mut next_period = period + 1;

    while is_same_troop(period, next_period) {
        next_period += 1;
        next_in += 60;
    }

    let changed_at = dt.clone() + Duration::minutes(next_in as i64);

    State {
        troop: get_troop_by_period(period),
        next_troop: get_troop_by_period(next_period),
        next_in: next_in,
        changed_at: changed_at
    }
}

fn is_same_troop(p1: usize, p2: usize) -> bool {
    get_troop_by_period(p1) == get_troop_by_period(p2)
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    #[test]
    fn test_get_state() {
        let dt = chrono::Local.ymd(2018, 9, 20).and_hms(15, 3, 15);
        let state = super::get_state(dt);
        assert_eq!(state.troop.name(), "闇朱の獣牙兵団");
        assert_eq!(state.next_troop.name(), "紫炎の鉄機兵団");
    }
}
