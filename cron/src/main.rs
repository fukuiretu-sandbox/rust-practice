use chrono::Local;
use cron::Schedule;
use std::str::FromStr;

fn main() {
    //               sec  min   hour   day of month   month   day of week   year
    // let expression = "0   30   9     1-30       May-Aug  Mon,Wed,Fri  2018/2";
    let expression = "0 0 9 * * 2-6 *";
    let schedule = Schedule::from_str(expression).unwrap();
    println!("Upcoming fire times:");
    for datetime in schedule.upcoming(Local).take(10) {
        println!("-> {}", datetime);
    }
}
