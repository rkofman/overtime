// fn foo_implementation() {
//     println!("Hello, world!");
// }

use chrono_tz::Tz;
use chrono::DateTime;

pub fn minutes_delta(start: DateTime<Tz>, end: DateTime<Tz>) -> i64 {
    (end - start).num_minutes()
}


#[cfg(test)]
mod tests;