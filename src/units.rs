#[derive(Debug, PartialEq)]
pub struct Minutes(pub i32);

#[derive(Debug, PartialEq)]
pub struct Cents(pub i32);

#[derive(Debug, PartialEq)]
pub struct LocalTime {
    // todo(rkofman): might be useful to use the "bounded_integer!" crate to enforce
    // boundary for hours and minutes between 0-23, and 0-59.
    // Remove todo if we conclusively decide not to do so.
    hours: i32,
    minutes: i32,
}