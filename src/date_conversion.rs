use crate::structs::date::Date;
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Local, NaiveDateTime, Utc};

pub fn convert_time_to_actual_date(time: u64) {
    let now: DateTime<Local> = Local::now();
    let seconds: u32 = (time % (60)) as u32;
    let minutes: u32 = ((time/60) % (60)) as u32;
    let hours: u32 = ((time/(60*60)) % 24) as u32; 
    let days: u32 = ((time/(60*24*60))) as u32; 
    println!("{}-{}:{}:{}", days, hours, minutes, seconds);
}
