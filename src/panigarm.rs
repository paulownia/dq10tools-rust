use chrono::{DateTime, Local, TimeZone, NaiveDateTime};
use chrono_tz::Asia::Tokyo;

#[derive(PartialEq, Eq)]
pub struct Boss {
   pub name: &'static str
}
const BIRD: Boss = Boss { name: "源世鳥アルマナ" };
const DRAGON: Boss = Boss { name: "じげんりゅう" };
const CATCHER: Boss = Boss { name: "パニガキャッチャー" };
const FRUITS: Boss = Boss { name: "源世果フルポティ" };
const CORVUS: Boss = Boss { name: "堕天使エルギオス" };
const PRISON: Boss = Boss { name: "源世妃フォルダイナ" };
const STAR: Boss = Boss { name: "魔妖星プルタヌス" };
const IRON: Boss = Boss { name: "鉄巨兵ダイダルモス" };

pub struct Event {
    pub boss: &'static Boss,
    pub started_at: DateTime<Local>,
}

// v7.3, 2025-01-29 6:00を起点とするスケジュール。三日周期でボスが変わる。
// 順番は変わらない想定
const SEQUENCE: [&Boss; 8] = [
    &BIRD,
    &DRAGON,
    &PRISON,
    &IRON,
    &CATCHER,
    &FRUITS,
    &STAR,
    &CORVUS,
];

pub fn get_current_event() -> Event {
    get_event(Local::now())
}

pub fn get_event<Tz: TimeZone>(dt: DateTime<Tz>) -> Event {
    let base_point = get_base_point();
    let calc_point = dt.naive_utc();
    let duration = calc_point - base_point;
    let period = (duration.num_days() / 3) as usize;
    let index = period % SEQUENCE.len();
    let boss = SEQUENCE[index];

    let utc_started_at = base_point + chrono::Duration::days(period as i64 * 3);
    let started_at = Local.from_utc_datetime(&utc_started_at);

    Event {
        boss,
        started_at,
    }
}

pub fn get_cycle<Tz: TimeZone>(dt: DateTime<Tz>) -> Vec<Event> {
    let mut result = Vec::new();
    for i in 0..SEQUENCE.len() {
        let boss = get_event(dt.clone() + chrono::Duration::days(i as i64 * 3));
        result.push(boss);
    }
    result
}

pub fn get_next_cycle() -> Vec<Event> {
    get_cycle(Local::now())
}

fn get_base_point() -> NaiveDateTime {
    // 2025-01-29 06:00:00 JST (v7.3 公開日)
    Tokyo.with_ymd_and_hms(2025, 1, 29, 6, 0, 0).unwrap().naive_utc()
}


#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    #[test]
    fn test_get_boss() {
        // 基準日の6時から3日周期でボスが変わる
        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2025, 1, 29, 13, 0, 0).single().unwrap();
        let event = super::get_event(dt);
        assert_eq!(event.boss.name, "源世鳥アルマナ");
        assert_eq!(event.started_at, tz.with_ymd_and_hms(2025, 1, 29, 6, 0, 0).single().unwrap());

        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2025, 2, 1, 5, 59, 59).single().unwrap();
        let event = super::get_event(dt);
        assert_eq!(event.boss.name, "源世鳥アルマナ");
        assert_eq!(event.started_at, tz.with_ymd_and_hms(2025, 1, 29, 6, 0, 0).single().unwrap());

        // 次のボスの切り替わり時間
        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2025, 2, 1, 6, 0, 0).single().unwrap();
        let event = super::get_event(dt);
        assert_eq!(event.boss.name, "じげんりゅう");
        assert_eq!(event.started_at, tz.with_ymd_and_hms(2025, 2, 1, 6, 0, 0).single().unwrap());

        // 繰り返しテスト, 24日後に同じボスが出現
        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2025, 2, 19, 6, 0, 0).single().unwrap();
        let event = super::get_event(dt);
        assert_eq!(event.boss.name, "堕天使エルギオス");
        assert_eq!(event.started_at, tz.with_ymd_and_hms(2025, 2, 19, 6, 0, 0).single().unwrap());

        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = dt + chrono::Duration::days(24);
        let event = super::get_event(dt);
        assert_eq!(event.boss.name, "堕天使エルギオス");
        assert_eq!(event.started_at, tz.with_ymd_and_hms(2025, 3, 15, 6, 0, 0).single().unwrap());
    }
}
