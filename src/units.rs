// the hourly_rate module may need to be rebranded.
// and the rest of the units moved into their own modules. perhaps:
// * Currency
// * Time
// * Complex
pub mod hourly_rate;

#[derive(Debug, PartialEq, Copy, Clone, derive_more::Add)]
pub struct Minutes(pub i64);

#[derive(Debug, PartialEq, Copy, Clone, derive_more::Add)]
pub struct Cents(pub i64);

#[derive(Debug, PartialEq)]
pub struct LocalTime {
    // todo(rkofman): might be useful to use the "bounded_integer!" crate to enforce
    // boundary for hours and minutes between 0-23, and 0-59.
    // Remove todo if we conclusively decide not to do so.
    pub hours: i64,
    pub minutes: i64,
}