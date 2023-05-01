use crate::units::{Minutes, Cents};
use rounded_div::RoundedDiv;

impl std::ops::Mul<HourlyWage> for Minutes {
    type Output = Cents;

    fn mul(self, hourly_rate: HourlyWage) -> Self::Output {
        hourly_rate * self // delegates to HourlyRate * Minutes
    }
}


#[derive(Debug, PartialEq, Copy, Clone, derive_more::Add)]
pub struct HourlyWage(i64); // TODO(rkofman): Make this polymorphic over currency types?

impl HourlyWage {
    pub fn from_cents_per_hour(num_cents:i64) -> HourlyWage {
        HourlyWage(num_cents)
    }

}

impl std::ops::Mul<Minutes> for HourlyWage {
    type Output = Cents;

    fn mul(self, minutes: Minutes) -> Self::Output {
        let minutes_per_hour = 60;
        let num_cents_per_hour= self.0;
        let num_minutes = minutes.0;

        // Always divide last because we're working with integers, and want to avoid
        // extraneous rounding which could throw off our maths.
        Cents((num_cents_per_hour * num_minutes).rounded_div(minutes_per_hour))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_math() {
        let rate = HourlyWage::from_cents_per_hour(1000);
        let time_worked = Minutes(60);
        assert_eq!(Cents(1000), rate * time_worked)
    }

    #[test]
    fn half_cent_rounds_up() {
        let rate = HourlyWage::from_cents_per_hour(5);
        let time_worked = Minutes(6);
        assert_eq!(Cents(1), rate * time_worked)
    }
}