mod date_conversion;
use date_conversion::convert_time_to_actual_date;
mod structs;
use structs::date::Date;

fn main() {
    let day: Date = Date::new(123142, String::from("Essen"));
    convert_time_to_actual_date(day);
}


