use chrono::Weekday;

use crate::units::LocalTime;

pub struct WorkWeekConfig {
    business_day_cutoff: LocalTime,
    business_week_cutoff: Weekday
}