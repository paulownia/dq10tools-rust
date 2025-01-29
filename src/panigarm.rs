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

pub fn get_current_boss() -> &'static Boss {
    get_boss(Local::now())
}

pub fn get_boss<Tz: TimeZone>(dt: DateTime<Tz>) -> &'static Boss {
    let base_point = get_base_point();
    let calc_point = dt.naive_utc();
    let duration = calc_point - base_point;
    let pass = (duration.num_days() / 3) as usize;
    let index = pass % SEQUENCE.len();
    SEQUENCE[index]
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
        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2025, 1, 29, 13, 0, 0).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "源世鳥アルマナ");

        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2025, 2, 1, 5, 59, 59).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "源世鳥アルマナ");

        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2025, 2, 1, 6, 0, 0).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "じげんりゅう");

        // 繰り返しテスト, 24日後に同じボスが出現
        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2025, 2, 19, 6, 0, 0).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "堕天使エルギオス");

        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2023, 3, 25, 6, 0,0).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "堕天使エルギオス");
    }
}
