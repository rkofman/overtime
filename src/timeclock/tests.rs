mod tests {

  use chrono::{TimeZone};
  use chrono_tz::US::Pacific;
  use super::super::*;
    
  #[test]
  fn test_duration() {
    let moment1 = Pacific.with_ymd_and_hms(2022, 10, 12, 9, 0, 0).unwrap();
    let moment2 = Pacific.with_ymd_and_hms(2022, 10, 12, 10, 30, 0).unwrap();
    assert_eq!(90, minutes_delta(moment1, moment2));
  }
}