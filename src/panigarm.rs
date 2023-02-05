use chrono::{DateTime, Local, TimeZone};

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


// 2023-01-29 6:00から（6.4は2/1から）
const SEQUENCE: [&Boss; 7] = [
    &FRUITS,
    &STAR,
    &CORVUS,
    &ALMANA,
    &DRAGON,
    &PRISON,
    &CATCHER,
];

pub fn get_current_boss() -> &'static Boss {
    get_boss(Local::now())
}

pub fn get_boss(dt: DateTime<Local>) -> &'static Boss {
    let base_point = get_base_point();
    let duration = dt - base_point;
    let pass = duration.num_weeks() as usize;
    let index = pass % SEQUENCE.len();
    SEQUENCE[index]
}

pub fn get_base_point() -> DateTime<Local> {
    Local.with_ymd_and_hms(2023, 1, 29, 6, 0, 0).single().unwrap()
}


#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    #[test]
    fn test_get_boss() {
        let dt = chrono::Local.with_ymd_and_hms(2023, 2, 5, 5, 59, 59).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "源世果フルポティ");

        let dt = chrono::Local.with_ymd_and_hms(2023, 2, 5, 6, 0, 0).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "魔妖星プルタヌス");

        let dt = chrono::Local.with_ymd_and_hms(2023, 2, 13, 6, 0, 0).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "堕天使エルギオス");
    }
}
