use chrono::Weekday;

use crate::units::LocalTime;

pub struct WorkWeekConfig {
    pub business_day_cutoff: LocalTime,
    pub business_week_cutoff: Weekday
}