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


const SEQUENCE: [&Boss; 6] = [
    &DRAGON,
    &PRISON,
    &CATCHER,
    &FRUITS,
    &CORVUS,
    &ALMANA,
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
    // バージョン6.3 2022-10-19 6時からの周期
    // バージョンアップ週（10/16 -）はじげんりゅう、翌週（10/23 - ）からフォルダイナ
    Local.with_ymd_and_hms(2022, 10, 16, 6, 0, 0).single().unwrap()
}


#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    #[test]
    fn test_get_boss() {
        let dt = chrono::Local.with_ymd_and_hms(2022, 10, 23, 5, 59, 59).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "じげんりゅう");

        let dt = chrono::Local.with_ymd_and_hms(2022, 10, 23, 6, 0, 0).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "源世妃フォルダイナ");

        let dt = chrono::Local.with_ymd_and_hms(2022, 11, 13, 6, 0, 0).single().unwrap();
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "堕天使エルギオス");
    }
}
