use chrono::{DateTime, Local, TimeZone, NaiveDateTime};
use chrono_tz::Asia::Tokyo;

#[derive(PartialEq, Eq)]
pub struct Boss {
   pub name: &'static str
}
const ALMANA: Boss = Boss { name: "源世鳥アルマナ" };
const DRAGON: Boss = Boss { name: "じげんりゅう" };
const CATCHER: Boss = Boss { name: "パニガキャッチャー" };
const FRUITS: Boss = Boss { name: "源世果フルポティ" };
const CORVUS: Boss = Boss { name: "堕天使エルギオス" };
const PRISON: Boss = Boss { name: "源世妃フォルダイナ" };
const STAR: Boss = Boss { name: "魔妖星プルタヌス" };
const IRON: Boss = Boss { name: "鉄巨兵ダイダルモス" };

// 2023-05-21 6:00を起点としたスケジュール
// v6.5は6/14から開始、新ボスの初回登場は6/18
const SEQUENCE: [&Boss; 8] = [
    &CORVUS,
    &ALMANA,
    &DRAGON,
    &PRISON,
    &IRON,
    &CATCHER,
    &FRUITS,
    &STAR,
];

pub fn get_current_boss() -> &'static Boss {
    get_boss(Local::now())
}

pub fn get_boss<Tz: TimeZone>(dt: DateTime<Tz>) -> &'static Boss {
    let base_point = get_base_point();
    let calc_point = dt.naive_utc();
    let duration = calc_point - base_point;
    let pass = duration.num_weeks() as usize;
    let index = pass % SEQUENCE.len();
    SEQUENCE[index]
}

fn get_base_point() -> NaiveDateTime {
    // 2023-05-21 06:00:00 JST
    Tokyo.with_ymd_and_hms(2023, 5, 21, 6, 0, 0).unwrap().naive_utc()
}


#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    #[test]
    fn test_get_boss() {
        // 切り替わりの境界テスト
        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2023, 7, 9, 5, 59, 59).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "源世果フルポティ");

        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2023, 7, 9, 6, 0, 0).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "魔妖星プルタヌス");

        // 繰り返しテスト
        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2023, 5, 21, 6, 0, 0).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "堕天使エルギオス");

        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2023, 7, 16, 12, 0, 0).single().unwrap(); // 7/23週に再度発生
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "堕天使エルギオス");

        // 新ボス初回
        let tz = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = tz.with_ymd_and_hms(2023, 6, 18, 6, 0, 0).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "鉄巨兵ダイダルモス");
    }
}
