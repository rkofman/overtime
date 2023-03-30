#![feature(is_some_and)]

mod timecard;
mod wage_summary;
mod units;
mod work_week;

use timecard::Timecard;
use wage_summary::{WageSummary};
use work_week::WorkWeekConfig;
pub mod timeclock;


pub fn calculate_wages(timecards: &Vec<Timecard>, _work_week_config: &WorkWeekConfig) -> WageSummary {
    let result = timecards.iter().fold(WageSummary::empty(), |acc, tc| -> WageSummary {
        let minutes = tc.minutes_worked();
        WageSummary::new(
            acc.regular_time + minutes,
            acc.overtime,
            acc.regular_wages + (minutes * tc.hourly_rate),
            acc.overtime_wages)
    });
    result
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Weekday, TimeZone, Days, Datelike, Duration};
    use chrono_tz::{US::Pacific, Tz};

    use crate::units::{LocalTime, Cents, Minutes, hourly_rate::HourlyRate};

    use super::*;

    fn datetime_for(weekday: Weekday, time: &str) -> DateTime<Tz> {
        let sunday_noon = Pacific.with_ymd_and_hms(2022, 10, 9, 0, 0, 0).unwrap();
        assert_eq!(sunday_noon.weekday(), Weekday::Sun);

        // TODO(rkofman): there has *got* to be a cleaner way of doing this. :shrug:
        let weekdays_starting_sunday = [
            Weekday::Sun, Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Thu, Weekday::Fri, Weekday::Sat
        ];
        let num_days = weekdays_starting_sunday.into_iter().position(|day| day == weekday).unwrap();
        let mut time_split = time.split(":");
        let num_hours = time_split.next().unwrap().parse::<i64>().unwrap();
        let num_minutes = time_split.next().unwrap().parse::<i64>().unwrap();

        sunday_noon + Days::new(num_days.try_into().unwrap()) + Duration::hours(num_hours) + Duration::minutes(num_minutes)
    }

    #[test]
    fn calculate_basic_wages() {

        let work_week_config = WorkWeekConfig {
            business_day_cutoff: LocalTime { hours: 2, minutes: 0 },
            business_week_cutoff: Weekday::Sun
        };

        let twenty_dollars_per_hour = HourlyRate::from_cents_per_hour(2000);

        let timecards = Vec::from([
            Timecard::new(
                datetime_for(Weekday::Mon, "09:00"),
                datetime_for(Weekday::Mon, "17:00"),
                twenty_dollars_per_hour).unwrap(),
            Timecard::new(
                datetime_for(Weekday::Tue, "09:30"),
                datetime_for(Weekday::Tue, "17:00"),
                twenty_dollars_per_hour).unwrap(),
        ]);

        let expected_wage = Cents((15.5*2000.0f64).round() as i64);
        let expected_wages = WageSummary::new(Minutes(15*60 + 30), Minutes(0), expected_wage, Cents(0));
        let result_wages = calculate_wages(&timecards, &work_week_config);
        assert_eq!(expected_wages.regular_time, result_wages.regular_time);
        assert_eq!(expected_wages.overtime, result_wages.overtime);
        assert_eq!(expected_wages.regular_wages, result_wages.regular_wages);
        assert_eq!(expected_wages.overtime_wages, result_wages.overtime_wages);
        assert_eq!(expected_wages, result_wages);
    }
}

/* other tests to try: */
        /*  given timecards with entries:
         (work week starts at 2AM on Sunday)
          Monday 9 am - 4pm // 7 hours
          Tuesday 9am - 5pm // 8 hours
          Wednesday 10am - 5pm // 7 hours
        */
        /*  given timecards with entries:
          Sunday (12midnight - 4am)
         (work week starts at 2AM on Sunday)
          Monday 9 am - 4pm // 7 hours
          Tuesday 9am - 5pm // 8 hours
          Wednesday 10am - 5pm // 7 hours
        */