use chrono::Timelike;
use crate::resources::Resource;
use crate::resources::chars::Char;

pub struct Clock { }

impl Clock {
    fn get_formatted_time() -> String {
        let date_time = chrono::offset::Local::now();
        let format = if date_time.second() % 2 == 0 { "%-I:%M" } else { "%-I %M" };
        date_time.time().format(format).to_string()
    }
}

impl Resource for Clock {
    fn new() -> Self {
        Self { }
    }

    fn get_resource(&self) -> Vec<[u32; 17]> {
        let date_time = Self::get_formatted_time();
        let mut result = [0u32; 17];
        let mut index = 1;
        for digit in date_time.chars() {
            let char_encoded = Char::get(digit);
            for val in &char_encoded {
                result[index] = *val;
                index += 1;
            }
            result[index] = 0;
            index += 1;
        }
        vec![result]
    }
}
