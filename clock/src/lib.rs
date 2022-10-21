use std::fmt;

const MINS_PER_HOUR: i32 = 60;
const MINS_PER_DAY: i32 = MINS_PER_HOUR * 24;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (minutes + hours * MINS_PER_HOUR).rem_euclid(MINS_PER_DAY);

        Clock {
            hours: total_minutes / MINS_PER_HOUR,
            minutes: total_minutes % MINS_PER_HOUR,
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
