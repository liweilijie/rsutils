use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::prelude::*;
use chrono::{Duration, ParseResult};
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

    // DateTime结构 转 毫秒
    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let millis: i64 = now.timestamp_millis();
    println!("current time millis:{}", millis); // millis:1648438531677

    // 毫秒时间与DateTime结构互转
    let dt: DateTime<Local> = chrono::Local.timestamp_millis(millis);
    println!("date time parsed millis:{}", dt); // 2022-03-28 11:39:39.202 +08:00

    // 格式化成字符串并且从字符串中解析出来
    let fmt = "%Y-%m-%d %H:%M:%S";

    let now: DateTime<Local> = Local::now();
    let dft: DelayedFormat<StrftimeItems> = now.format(fmt);
    let str_date: String = dft.to_string();
    println!("{}", str_date); // 2022-03-28 11:46:29

    // 从字符串中解析出来
    let result: ParseResult<NaiveDateTime> = NaiveDateTime::parse_from_str(str_date.as_str(), fmt);
    if result.is_err() {
        result.expect("parse error");
    }
    let date: NaiveDateTime = result.unwrap();
    println!("time:{}, parsed from:'{}'", date, str_date); // time:2022-03-28 11:54:37, parsed from:'2022-03-28 11:54:37'
    elapsed();

    checked_add_sub_signed();
    time_zone_transfer();
}

fn elapsed() {
    let start = std::time::Instant::now();
    // to do calculation
    std::thread::sleep(std::time::Duration::from_secs(1));
    let duration: std::time::Duration = start.elapsed();
    println!("time elapsed in to do work is: {:?}", duration); // 5.005039875s
}

fn day_earlier(date_time: DateTime<Local>) -> Option<DateTime<Local>> {
    date_time.checked_sub_signed(chrono::Duration::days(1))
}

fn checked_add_sub_signed() {
    let now: DateTime<Local> = Local::now();
    // now:2022-03-28 14:26:35.189780 +08:00
    println!("now:{}", now);
    // 用今天时间加上2周再加上1周再减去1天的日期
    let almost_three_weeks_from_now: Option<DateTime<Local>> = now.checked_add_signed(chrono::Duration::weeks(2))
    .and_then(|in_2weeks: DateTime<Local>| in_2weeks.checked_add_signed(chrono::Duration::weeks(1)))
    .and_then(day_earlier);

    // 2022-04-17 14:26:35.189780 +08:00
    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    // We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center.
    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }
}

// 使用 offset::Local::now 获取本地时间并显示，
// 然后使用 DateTime::from_utc 结构体方法将其转换为 UTC 标准格式。
// 最后，使用 offset::FixedOffset 结构体，可以将 UTC 时间转换为 UTC+8 和 UTC-2。
fn time_zone_transfer() {
    let local_time = Local::now();
    let utc_time = DateTime::<chrono::Utc>::from_utc(local_time.naive_utc(), chrono::Utc);
    let china_timezone = FixedOffset::east(8 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    println!("local time:{}", local_time); // local time:2022-03-28 14:34:33.756485 +08:00
    println!("utc time:{}", utc_time); // utc time:2022-03-28 06:34:33.756485 UTC
    // Time in Hong Kong now is 2022-03-28 14:34:33.756485 +08:00
    println!("Time in Hong Kong now is {}", utc_time.with_timezone(&china_timezone));
    // Time in Rio now is 2022-03-28 04:34:33.756485 -02:00
    println!("Time in Rio now is {}", utc_time.with_timezone(&rio_timezone));
}