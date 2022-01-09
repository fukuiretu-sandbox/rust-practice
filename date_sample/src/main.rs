use std::thread::sleep;

use chrono::{Duration, Local};

fn main() {
    // 現在時刻の取得
    let dt1 = Local::now();
    println!("{}", dt1);

    // 時刻の加算
    let dt2 = dt1 + Duration::seconds(5);
    println!("{}", dt2);

    sleep(std::time::Duration::new(3, 0));

    // 現在時刻が5秒以内に収まっているか否かを判定する
    let duration: Duration = dt2 - Local::now();
    println!("secs: {}", duration.num_seconds());

    if duration.num_seconds() > 0 {
        println!("alive!")
    } else {
        println!("dead!")
    }
}
