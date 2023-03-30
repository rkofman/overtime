use rust_decimal::{Decimal, prelude::{FromPrimitive, ToPrimitive}, RoundingStrategy};
use rust_decimal_macros::dec;
use crate::units::{Minutes, Cents};

impl std::ops::Mul<HourlyRate> for Minutes {
    type Output = Cents;

    fn mul(self, hourly_rate: HourlyRate) -> Self::Output {
        hourly_rate * self // delegate to HourlyRate * Minutes
    }
}


#[derive(Debug, PartialEq, Copy, Clone, derive_more::Add)]
pub struct HourlyRate(i64); // TODO(rkofman): Make this polymorphic over currency types.

impl HourlyRate {
    pub fn from_cents_per_hour(num_cents:i64) -> HourlyRate {
        HourlyRate(num_cents)
    }

}

impl std::ops::Mul<Minutes> for HourlyRate {
    type Output = Cents;

    fn mul(self, minutes: Minutes) -> Self::Output {
        let minutes_per_hour = dec!(60);
        let num_cents_per_hour= Decimal::from_i64(self.0).unwrap();
        let num_minutes = Decimal::from_i64(minutes.0).unwrap();
        let rounding_strat = RoundingStrategy::MidpointAwayFromZero;

        Cents((num_cents_per_hour / minutes_per_hour * num_minutes)
            // Have to round to a useful number of sig figs *first* before rounding
            // to cents. Otherwise, any division and multiplication by 3 can have
            // surprising effects (0.499999999 doesn't round up, but 0.50 does)
            .round_dp_with_strategy(2, rounding_strat)
            .round_dp_with_strategy(0, rounding_strat)
            .to_i64()
            .unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_math() {
        let rate = HourlyRate::from_cents_per_hour(1000);
        let time_worked = Minutes(60);
        assert_eq!(Cents(1000), rate * time_worked)
    }

    #[test]
    fn half_cent_rounds_up() {
        let rate = HourlyRate::from_cents_per_hour(5);
        let time_worked = Minutes(6);
        assert_eq!(Cents(1), rate * time_worked)
    }
}