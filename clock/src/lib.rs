use std::fmt;

const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(new_hours: i32, new_minutes: i32) -> Self {
        let mut minutes = new_minutes % MINUTES_PER_HOUR;
        let mut hours = new_hours + new_minutes / MINUTES_PER_HOUR;

        if minutes < 0 {
            hours -= 1;
            minutes += 60;
        }

        Clock {
            hours: hours.rem_euclid(HOURS_PER_DAY),
            minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
