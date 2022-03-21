use chrono::prelude::*;
use chrono::{Local, Timelike};

fn main() {
    println!("Hello, world!");
    println!("now.timestamp:{}", chrono::Local::now().timestamp());
    println!("now:{}", chrono::Local::now().second());

    let a1 = Local.ymd(2020, 3, 1).and_hms(5, 20, 0);
    let a2 = Local.ymd(2020, 3, 1).and_hms(7, 20, 0);
    println!("a1:{}", a1);
    println!("a1:{}", a1.timestamp());

    let m1 = a1.timestamp();
    let m2 = a2.timestamp();

    println!("beyond:{}", m2 - m1);

}
