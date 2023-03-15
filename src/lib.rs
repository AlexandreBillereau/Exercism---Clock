const MIN_IN_ONE_H: i32 = 60;
const HOURS_IN_DAY: i32 = 24;

#[derive(Debug)]
pub struct Clock{
    hours: i32, 
    minutes: i32,
}

impl Clock {

    pub fn new(hours: i32, minutes: i32) -> Self {

        
        let hours = Clock::convert_hours_in_24h_range(hours);
        let (hours, minutes) = Clock::convert_to_hours(hours, minutes);
        let hours = Clock::convert_hours_in_24h_range(hours);
        
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) = Clock::convert_to_hours(self.hours, minutes + self.minutes);
        let hours = Clock::convert_hours_in_24h_range(hours);
        Clock { hours, minutes } 
    }

    pub fn convert_hours_in_24h_range(hours: i32) -> i32{
        let result : i32;
        if is_positive(hours) {
            result = hours % HOURS_IN_DAY;
        }else{
            result = ((hours % HOURS_IN_DAY) + HOURS_IN_DAY) % HOURS_IN_DAY;
        }

        result
    }

    pub fn convert_to_hours(hours: i32, minutes: i32) -> (i32,i32) {
        let total_hours  : i32;
        let total_minutes: i32;
        let hours_into_min: i32 = hours * MIN_IN_ONE_H + minutes;

        if is_positive(minutes) {
            total_hours = (hours_into_min as f32 / 60.0).floor() as i32;
            total_minutes = hours_into_min % MIN_IN_ONE_H;   
        }else{
            total_hours = (hours_into_min as f32 / 60.0).floor() as i32;
            total_minutes = ((hours_into_min % MIN_IN_ONE_H) + MIN_IN_ONE_H) % MIN_IN_ONE_H;   
        }
        (total_hours, total_minutes)
    }   

    pub fn to_string(&self) -> String{
        
        let mut result: String;
        if self.hours > 9 {
            result = format!("{}:", self.hours);
        }else{
            result = format!("0{}:", self.hours);
        }

        if self.minutes > 9 {
            result.push_str(&format!("{}", self.minutes));
        }else {
            result.push_str(&format!("0{}", self.minutes));
        }

        return result;
     
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes && self.hours == other.hours
    }
}

fn is_positive(n: i32) -> bool {
    n > -1
}

fn not_in_range(minutes: i32) -> bool {
    !(minutes >= 0 && minutes <= 60)
}

