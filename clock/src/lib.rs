use std::fmt;

const MINUTES_IN_HOUR: i64 = 60;
const MINUTES_IN_DAY: i64 = 24 * MINUTES_IN_HOUR;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        Clock {
            minutes: (((hours * MINUTES_IN_HOUR + minutes) /* Convert hours to minutes and get total minutes */
            % MINUTES_IN_DAY) /* Account for if total minutes have overflown over a day */
            + MINUTES_IN_DAY) /* Add minutes in a day in case total minutes are negative */
            % MINUTES_IN_DAY, /* Again, account for if total minutes have overflown over a day */
        }
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{hours:02}:{minutes:02}",
            hours = self.minutes / MINUTES_IN_HOUR,
            minutes = self.minutes % MINUTES_IN_HOUR
        )
    }
}
