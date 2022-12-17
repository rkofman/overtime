use chrono::TimeZone;
use chrono_tz::US::Pacific;

mod timeclock;
fn main() {
    let moment1 = Pacific.with_ymd_and_hms(2022, 10, 12, 9, 0, 0).unwrap();
    let moment2 = Pacific.with_ymd_and_hms(2022, 10, 12, 10, 30, 0).unwrap();
    timeclock::minutes_delta(moment1, moment2);
}