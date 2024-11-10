use crate::resources::Resource;
use crate::resources::chars::Char;

const MAX_TICK: u32 = 60;
pub struct Clock {
    tick: u32,
    cur_time_str: String
}
impl Clock {
    fn get_formatted_time(&self) -> String {
        let date_time = chrono::offset::Local::now();
        let format = if self.tick < MAX_TICK / 2 { "%-I:%M" } else { "%-I %M" };
        date_time.time().format(format).to_string()
    }

    pub fn update_tick(&mut self) {
        self.tick += 1;
        self.tick %= MAX_TICK;
    }
}

impl Resource for Clock {
    fn new() -> Self {
        Clock { tick: 0, cur_time_str: String::new() }
    }

    fn get_resource(&mut self) -> Vec<[u32; 17]> {
        let date_time = self.get_formatted_time();
        if self.cur_time_str == date_time {
            return vec![];
        }
        let mut result = [0u32; 17];
        let mut index = 1;
        for digit in date_time.chars() {
            let char_encoded = Char::get(digit);
            char_encoded.iter().for_each(|val| {
                result[index] = *val;
                index += 1;
            });
            result[index] = 0;
            index += 1;
        }
        vec![result]
    }
}
