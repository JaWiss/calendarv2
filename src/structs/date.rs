
#[derive(Debug)]
pub struct Date {
    time: u64,
    occasion: String
}
impl Date {
    pub fn new(time: u64, occasion: String) -> Date {
        Date {
            time,
            occasion
        }
    }
}
