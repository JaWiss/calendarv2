use crate::structs::date::Date;
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Local, NaiveDateTime, Utc};

pub fn convert_time_to_actual_date(time: u64) {
    let seconds: u32 = (time % (60)) as u32;
    let minutes: u32 = ((time/60) % (60)) as u32;
    let hours: u32 = ((time/(60*60)) % 24) as u32; 
    let mut days: i32 = ((time/(60*24*60))) as i32; 
    println!("{}-{}:{}:{}", days, hours, minutes, seconds);
    let year: u32= find_actual_year(&mut days); 
    println!("{}-{}-{}:{}:{}", year, days, hours, minutes, seconds);
}

pub fn find_actual_year(days: &mut i32) -> u32 {
    let mut current_year: u32 = 2020;
    while(true) {
        *days = *days - days_in_year(current_year); 
        if *days > 0 {
            current_year += 1;
            continue;
        }
        *days = *days + days_in_year(current_year);
        return current_year;
    }
    return 0;
}

pub fn days_in_year(year: u32) -> i32 {
    if year % 4 != 0 {
        return 365;
    } else if year % 400 == 0 {
        return 366;
    } else if year % 100 == 0 {
        return 365;
    } else {
        return 366;
    }
}
