use scrypto::math::BnumI256;
use scrypto::prelude::{dec, Decimal};
use std::fmt;
use std::ops::Neg;

/// A constant equal to `e = exp(1)` also known as Euler number.
pub const EULER_NUMBER: Decimal = Decimal(BnumI256::from_digits([2718281828459045235, 0, 0, 0]));

/// Returns the exponential of a [`Decimal`] using Taylor series
///
/// # Arguments
/// * `value` - The Decimal to compute the exponential for
///
/// # Examples
///
/// ```
/// use scrypto::prelude::Decimal;
/// use stoichiometric::decimal_maths::exp;
///
/// let res = exp(Decimal::one());
/// let true_res = Decimal::from(1.0_f64.exp().to_string());
/// let diff = res - true_res;
/// assert!(diff.abs() < Decimal::from("0.000000000000001"));
/// ```
pub fn exp<T: TryInto<Decimal>>(value: T) -> Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
{
    let value = value.try_into().expect("Cannot convert to Decimal");
    if value.is_zero() {
        return Decimal::ONE;
    } else if value.is_negative() {
        return if value < dec!(-43) {
            // Because exp(-43)< 10^-18, the smallest representable Decimal, we return 0
            Decimal::zero()
        } else {
            Decimal::ONE / exp(value.neg())
        };
    } else {
        // The Taylor series of exp is : exp(x) = SUM x^k / fact(k)

        // outputted result
        let mut result = Decimal::one();

        // next term of the taylor expansion
        let mut added_term = value;

        // counter to remember the index of the next term to add
        let mut counter = Decimal::one();

        while added_term != Decimal::zero() {
            result = result + added_term;

            counter = counter + 1;
            // previous term was x^k/k! so the next term is:  previous*x/(k+1)
            let mut next_term = added_term / counter;
            next_term = next_term * value;

            added_term = next_term;
        }

        result
    }
}

/// Returns the natural logarithm of a [`Decimal`] using Halley's method
///
/// # Arguments
/// * `value` - The Decimal to compute the logarithm for
///
/// # Examples
///
/// ```
/// use scrypto::prelude::Decimal;
/// use stoichiometric::decimal_maths::ln;
///
/// let res = ln(100);
/// let true_res = Decimal::from(100.0_f64.ln().to_string());
/// let diff = res - true_res;
/// assert!(diff.abs() < Decimal::from("0.000000000000001"));
/// ```
pub fn ln<T: TryInto<Decimal>>(value: T) -> Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
{
    let mut value = value.try_into().expect("Cannot convert to Decimal");
    assert!(
        value.is_positive(),
        "Logarithm is only defined for positive numbers"
    );

    // We are not using the Taylor expansion of ln because it converges too slowly
    // To compute ln(y), we use Halley's method and we compute the sequence x_n defined by induction:
    // x_{n+1} = x_n + ( y - exp(x_n) )/( y + exp(x_n) )

    // Because, exp overflows very quickly, we rewrite y = a*e^n with a<e
    // Therefore, ln(y) = ln(a) + n

    if value < Decimal::ONE {
        return -ln(Decimal::ONE / value)
    }

    let mut n = 0;
    while value >= EULER_NUMBER {
        value = value / EULER_NUMBER;
        n += 1;
    }

    if value  == Decimal::ONE
    {
        return Decimal::from(n)
    }

    // Start with an arbitrary number as the first guess
    let mut result = value / Decimal::from(2u8);

    // Too small to represent, so we start with self
    // Future iterations could actually avoid using a decimal altogether and use a buffered
    // vector, only combining back into a decimal on return
    if result.is_zero() {
        result = value;
    }
    let mut last = result + 1;

    // Keep going while last and result are not equal
    let mut circuit_breaker = 0;
    while last != result {
        circuit_breaker += 1;
        assert!(circuit_breaker < 1000, "geo mean circuit breaker");

        last = result;
        let exp_last = exp(last);
        result = last + (value - exp_last) / (value + exp_last) * 2;
    }

    result + Decimal::from(n)
}

pub fn pow<T: TryInto<Decimal>, E: TryInto<Decimal>>(value: T, exponent: T) -> Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
    <E as TryInto<Decimal>>::Error: fmt::Debug,
{
    let value_dec = value.try_into().expect("Cannot convert into Decimal");
    let exp_dec = exponent.try_into().expect("Cannot convert into Decimal");
    assert!(
        !exp_dec.is_zero() || !value_dec.is_zero(),
        "O^O is undefined"
    );

    exp(exp_dec * ln(value_dec))
}

#[cfg(test)]
mod tests {
    use crate::decimal_maths::{exp, ln, EULER_NUMBER};
    use rand::Rng;
    use scrypto::math::{Decimal};
    use scrypto::prelude::dec;

    #[test]
    fn test_exp_zero() {
        let res = exp(0);
        let true_res = Decimal::one();
        assert_eq!(res, true_res);
    }

    #[test]
    fn test_exp_random_pos() {
        let num: f64 = rand::thread_rng().gen_range(0.0..2.0);
        let dec_num = Decimal::from(num.to_string());
        let res = exp(dec_num);
        let true_res = Decimal::from(num.exp().to_string());
        let diff = res - true_res;
        let acceptable_difference = 10e-14;
        assert!(
            diff.abs() < Decimal::from(acceptable_difference.to_string()),
            "{}, {}",
            res,
            true_res
        );
    }

    #[test]
    fn test_exp_random_neg() {
        let num: f64 = rand::thread_rng().gen_range(-2.0..0.0);
        let dec_num = Decimal::from(num.to_string());
        let res = exp(dec_num);
        let true_res = Decimal::from(num.exp().to_string());
        let diff = res - true_res;
        let acceptable_difference = 10e-14;
        assert!(
            diff.abs() < Decimal::from(acceptable_difference.to_string()),
            "{}, {}",
            res,
            true_res
        );
    }

    #[test]
    #[should_panic]
    fn test_ln_neg() {
        let _m = ln(-5);
    }

    #[test]
    #[should_panic]
    fn test_ln_zero() {
        let _m = ln(0);
    }

    #[test]
    fn test_ln_int() {
        let res = ln(exp(12));
        let true_res = dec!(12);
        let diff = res - true_res;
        assert!(diff.abs() < Decimal::from("0.000000000000001"));
    }

    #[test]
    fn test_ln_euler() {
        let res = ln(EULER_NUMBER);
        assert_eq!(res, Decimal::one())
    }

    #[test]
    fn test_ln_random() {
        let num: f64 = rand::thread_rng().gen_range(0.0..10000.0);
        let dec_num = Decimal::from(num.to_string());
        let res = ln(dec_num);
        let true_res = Decimal::from(num.ln().to_string());
        let diff = res - true_res;
        let acceptable_difference = 10e-14;
        assert!(
            diff.abs() < Decimal::from(acceptable_difference.to_string()),
            "{}, {}",
            res,
            true_res
        );
    }
}
