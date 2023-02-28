#[derive(Debug, PartialEq, Copy, Clone, derive_more::Add)]
pub struct Minutes(pub i64);

#[derive(Debug, PartialEq, Copy, Clone, derive_more::Add)]
pub struct DecimalMinutes(pub i64);

impl std::ops::Mul<Cents> for DecimalMinutes {
    type Output = Cents;

    fn mul(self, rhs: Cents) -> Self::Output {
        assert_eq!(0, self.0 * rhs.0 % 100);
        Cents(self.0 * rhs.0 / 100)
    }
}


#[derive(Debug, PartialEq, Copy, Clone, derive_more::Add)]
pub struct Cents(pub i64);

impl std::ops::Mul<DecimalMinutes> for Cents {
    type Output = Cents;

    fn mul(self, rhs: DecimalMinutes) -> Self::Output {
        rhs * self // delegate to DecimalMinutes * Cents
    }
}

#[derive(Debug, PartialEq)]
pub struct LocalTime {
    // todo(rkofman): might be useful to use the "bounded_integer!" crate to enforce
    // boundary for hours and minutes between 0-23, and 0-59.
    // Remove todo if we conclusively decide not to do so.
    pub hours: i64,
    pub minutes: i64,
}