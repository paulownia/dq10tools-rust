use chrono::{DateTime, Local, TimeZone};

#[derive(PartialEq, Eq)]
pub struct Boss {
   pub name: &'static str
}
const ALMANA: Boss = Boss { name: "源世鳥アルマナ" };
const DRAGON: Boss = Boss { name: "じげんりゅう" };
const FRUITS: Boss = Boss { name: "源世果フルポティ" };
const CORVUS: Boss = Boss { name: "堕天使エルギオス" };

const SEQUENCE: [&Boss; 4] = [
    &FRUITS,
    &CORVUS,
    &ALMANA,
    &DRAGON,
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
    Local.ymd(2022, 3, 20).and_hms(6, 0, 0)
}


#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    #[test]
    fn test_get_boss() {
        let dt = chrono::Local.ymd(2022, 4, 9).and_hms(5, 59, 59);
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "源世鳥アルマナ");

        let dt = chrono::Local.ymd(2022, 4, 10).and_hms(6, 0, 0);
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "じげんりゅう");

        let dt = chrono::Local.ymd(2022, 3, 27).and_hms(6, 0, 0);
        let boss = super::get_boss(dt);
        assert_eq!(boss.name, "堕天使エルギオス");
    }
}
