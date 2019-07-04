use std::time::SystemTime;
use serde_json::json;

fn main() {
  const MILLIS_ONE_DAY : u64 = 86400000;
  const BASE_POINT : u64  = 1467752400000;   // 2016-07-06 06:00:00
  match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(n) => {
        let d = ((n.as_millis() as u64) - BASE_POINT) / MILLIS_ONE_DAY;

        let json = json!({
            "レグナード": (d + 2) % 4 + 1,
            "ダークキング": d % 4 + 1,
            "メイヴ": (d + 3) % 4 + 1
        });
        println!("{}", json.to_string());
    },
    Err(_) => panic!("SystemTime before UNIX EPOCH"),
  }
}
