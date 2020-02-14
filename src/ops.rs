// Copyright 2020 CoD Team

//! Implementing operators for numeric.

use crate::NumericVar;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// The main implementation
// &self + &other
impl Add<&NumericVar> for &NumericVar {
    type Output = NumericVar;

    #[inline]
    fn add(self, other: &NumericVar) -> Self::Output {
        NumericVar::add(self, other)
    }
}

// self + &other
impl Add<&NumericVar> for NumericVar {
    type Output = NumericVar;

    #[inline]
    fn add(self, other: &NumericVar) -> Self::Output {
        Add::add(&self, other)
    }
}

// self + other
impl Add<NumericVar> for NumericVar {
    type Output = NumericVar;

    #[inline]
    fn add(self, other: NumericVar) -> Self::Output {
        Add::add(&self, &other)
    }
}

// &self + other
impl Add<NumericVar> for &NumericVar {
    type Output = NumericVar;

    #[inline]
    fn add(self, other: NumericVar) -> Self::Output {
        Add::add(self, &other)
    }
}

// &mut self += &other
impl AddAssign<&NumericVar> for NumericVar {
    #[inline]
    fn add_assign(&mut self, other: &NumericVar) {
        let result = Add::add(self as &NumericVar, other);
        *self = result;
    }
}

// &mut self += other
impl AddAssign<NumericVar> for NumericVar {
    #[inline]
    fn add_assign(&mut self, other: NumericVar) {
        let result = Add::add(self as &NumericVar, &other);
        *self = result;
    }
}

// The main implementation
// &self - &other
impl Sub<&NumericVar> for &NumericVar {
    type Output = NumericVar;

    #[inline]
    fn sub(self, other: &NumericVar) -> Self::Output {
        NumericVar::sub(self, other)
    }
}

// self - &other
impl Sub<&NumericVar> for NumericVar {
    type Output = NumericVar;

    #[inline]
    fn sub(self, other: &NumericVar) -> Self::Output {
        Sub::sub(&self, other)
    }
}

// self - other
impl Sub<NumericVar> for NumericVar {
    type Output = NumericVar;

    #[inline]
    fn sub(self, other: NumericVar) -> Self::Output {
        Sub::sub(&self, &other)
    }
}

// &self - other
impl Sub<NumericVar> for &NumericVar {
    type Output = NumericVar;

    #[inline]
    fn sub(self, other: NumericVar) -> Self::Output {
        Sub::sub(self, &other)
    }
}

// &mut self -= &other
impl SubAssign<&NumericVar> for NumericVar {
    #[inline]
    fn sub_assign(&mut self, other: &NumericVar) {
        let result = Sub::sub(self as &NumericVar, other);
        *self = result;
    }
}

// &mut self -= other
impl SubAssign<NumericVar> for NumericVar {
    #[inline]
    fn sub_assign(&mut self, other: NumericVar) {
        let result = Sub::sub(self as &NumericVar, &other);
        *self = result;
    }
}

// The main implementation
// &self * &other
impl Mul<&NumericVar> for &NumericVar {
    type Output = NumericVar;

    #[inline]
    fn mul(self, other: &NumericVar) -> Self::Output {
        // we request exact representation for the product,
        // rscale = sum(dscale of self, dscale of other)
        let rscale = self.dscale + other.dscale;
        NumericVar::mul(self, other, rscale)
    }
}

// self * &other
impl Mul<&NumericVar> for NumericVar {
    type Output = NumericVar;

    #[inline]
    fn mul(self, other: &NumericVar) -> Self::Output {
        Mul::mul(&self, other)
    }
}

// self * other
impl Mul<NumericVar> for NumericVar {
    type Output = NumericVar;

    #[inline]
    fn mul(self, other: NumericVar) -> Self::Output {
        Mul::mul(&self, &other)
    }
}

// &self * other
impl Mul<NumericVar> for &NumericVar {
    type Output = NumericVar;

    #[inline]
    fn mul(self, other: NumericVar) -> Self::Output {
        Mul::mul(self, &other)
    }
}

// &mut self *= &other
impl MulAssign<&NumericVar> for NumericVar {
    #[inline]
    fn mul_assign(&mut self, other: &NumericVar) {
        let result = Mul::mul(self as &NumericVar, other);
        *self = result;
    }
}

// &mut self *= other
impl MulAssign<NumericVar> for NumericVar {
    #[inline]
    fn mul_assign(&mut self, other: NumericVar) {
        let result = Mul::mul(self as &NumericVar, &other);
        *self = result;
    }
}

// The main implementation
// &self / &other
impl Div<&NumericVar> for &NumericVar {
    type Output = NumericVar;

    #[inline]
    fn div(self, other: &NumericVar) -> Self::Output {
        self.checked_div(other).expect("attempt to divide by zero")
    }
}

// &self / other
impl Div<NumericVar> for &NumericVar {
    type Output = NumericVar;

    #[inline]
    fn div(self, other: NumericVar) -> Self::Output {
        Div::div(self, &other)
    }
}

// self / &other
impl Div<&NumericVar> for NumericVar {
    type Output = NumericVar;

    #[inline]
    fn div(self, other: &NumericVar) -> Self::Output {
        Div::div(&self, other)
    }
}

// self / other
impl Div<NumericVar> for NumericVar {
    type Output = NumericVar;

    #[inline]
    fn div(self, other: NumericVar) -> Self::Output {
        Div::div(&self, &other)
    }
}

// &mut self /= &other
impl DivAssign<&NumericVar> for NumericVar {
    #[inline]
    fn div_assign(&mut self, other: &NumericVar) {
        let result = Div::div(self as &NumericVar, other);
        *self = result;
    }
}

// &mut self /= other
impl DivAssign<NumericVar> for NumericVar {
    #[inline]
    fn div_assign(&mut self, other: NumericVar) {
        let result = Div::div(self as &NumericVar, &other);
        *self = result;
    }
}

#[cfg(test)]
mod tests {
    use crate::NumericVar;

    fn assert_add(val1: &str, val2: &str, expected: &str) {
        let var1 = val1.parse::<NumericVar>().unwrap();
        let var2 = val2.parse::<NumericVar>().unwrap();

        let result1 = &var1 + &var2;
        assert_eq!(result1.to_string(), expected);

        let result2 = &var2 + &var1;
        assert_eq!(result2.to_string(), expected);

        let mut result3 = var1.clone();
        result3 += &var2;
        assert_eq!(result3.to_string(), expected);

        let mut result4 = var2.clone();
        result4 += &var1;
        assert_eq!(result4.to_string(), expected);
    }

    #[test]
    fn add() {
        assert_add("NaN", "10000.00001", "NaN");
        assert_add("NaN", "00000.00000", "NaN");
        assert_add("NaN", "-10000.00001", "NaN");
        assert_add("0.000000001", "100000000", "100000000.000000001");
        assert_add("123456789.987654321", "-123456789.987654321", "0.000000000");
        assert_add("987654321.123456789", "-987654321.123456789", "0.000000000");
        assert_add(
            "123456789.987654321",
            "987654321.123456789",
            "1111111111.111111110",
        );
        assert_add("123456789.987654321", "00000.00000", "123456789.987654321");
        assert_add(
            "123456789.987654321",
            "-987654321.123456789",
            "-864197531.135802468",
        );
        assert_add("00000.00000", "987654321.123456789", "987654321.123456789");
        assert_add("00000.00000", "00000.00000", "0.00000");
        assert_add(
            "00000.00000",
            "-987654321.123456789",
            "-987654321.123456789",
        );
        assert_add(
            "-123456789.987654321",
            "987654321.123456789",
            "864197531.135802468",
        );
        assert_add(
            "-123456789.987654321",
            "00000.00000",
            "-123456789.987654321",
        );
        assert_add(
            "-123456789.987654321",
            "-987654321.123456789",
            "-1111111111.111111110",
        );
    }

    fn assert_sub(val1: &str, val2: &str, expected1: &str, expected2: &str) {
        let var1 = val1.parse::<NumericVar>().unwrap();
        let var2 = val2.parse::<NumericVar>().unwrap();

        let result1 = &var1 - &var2;
        assert_eq!(result1.to_string(), expected1);

        let result2 = &var2 - &var1;
        assert_eq!(result2.to_string(), expected2);

        let mut result3 = var1.clone();
        result3 -= &var2;
        assert_eq!(result3.to_string(), expected1);

        let mut result4 = var2.clone();
        result4 -= &var1;
        assert_eq!(result4.to_string(), expected2);
    }

    #[test]
    fn sub() {
        assert_sub("NaN", "10000.00001", "NaN", "NaN");
        assert_sub("NaN", "00000.00000", "NaN", "NaN");
        assert_sub("NaN", "-10000.00001", "NaN", "NaN");
        assert_sub(
            "0.000000001",
            "100000000",
            "-99999999.999999999",
            "99999999.999999999",
        );
        assert_sub(
            "123456789.987654321",
            "123456789.987654321",
            "0.000000000",
            "0.000000000",
        );
        assert_sub(
            "987654321.123456789",
            "987654321.123456789",
            "0.000000000",
            "0.000000000",
        );
        assert_sub(
            "123456789.987654321",
            "987654321.123456789",
            "-864197531.135802468",
            "864197531.135802468",
        );
        assert_sub(
            "123456789.987654321",
            "00000.00000",
            "123456789.987654321",
            "-123456789.987654321",
        );
        assert_sub(
            "123456789.987654321",
            "-987654321.123456789",
            "1111111111.111111110",
            "-1111111111.111111110",
        );
        assert_sub(
            "00000.00000",
            "987654321.123456789",
            "-987654321.123456789",
            "987654321.123456789",
        );
        assert_sub("00000.00000", "00000.00000", "0.00000", "0.00000");
        assert_sub(
            "00000.00000",
            "-987654321.123456789",
            "987654321.123456789",
            "-987654321.123456789",
        );
        assert_sub(
            "-123456789.987654321",
            "987654321.123456789",
            "-1111111111.111111110",
            "1111111111.111111110",
        );
        assert_sub(
            "-123456789.987654321",
            "00000.00000",
            "-123456789.987654321",
            "123456789.987654321",
        );
        assert_sub(
            "-123456789.987654321",
            "-987654321.123456789",
            "864197531.135802468",
            "-864197531.135802468",
        );
    }

    fn assert_mul(val1: &str, val2: &str, expected: &str) {
        let var1 = val1.parse::<NumericVar>().unwrap();
        let var2 = val2.parse::<NumericVar>().unwrap();

        let result1 = &var1 * &var2;
        assert_eq!(result1.to_string(), expected);

        let result2 = &var2 * &var1;
        assert_eq!(result2.to_string(), expected);

        let mut result3 = var1.clone();
        result3 *= &var2;
        assert_eq!(result3.to_string(), expected);

        let mut result4 = var2.clone();
        result4 *= &var1;
        assert_eq!(result4.to_string(), expected);
    }

    #[test]
    fn mul() {
        assert_mul("NaN", "10000.00001", "NaN");
        assert_mul("NaN", "00000.00000", "NaN");
        assert_mul("NaN", "-10000.00001", "NaN");
        assert_mul("0.000000001", "100000000", "0.100000000");
        assert_mul(
            "123456789.987654321",
            "-123456789.987654321",
            "-15241578994055784.200731595789971041",
        );
        assert_mul(
            "987654321.123456789",
            "-987654321.123456789",
            "-975461058033836303.240512116750190521",
        );
        assert_mul(
            "123456789.987654321",
            "987654321.123456789",
            "121932632103337905.662094193112635269",
        );
        assert_mul("123456789.987654321", "00000.00000", "0.00000000000000");
        assert_mul(
            "123456789.987654321",
            "-987654321.123456789",
            "-121932632103337905.662094193112635269",
        );
        assert_mul("00000.00000", "987654321.123456789", "0.00000000000000");
        assert_mul("00000.00000", "00000.00000", "0.0000000000");
        assert_mul("00000.00000", "-987654321.123456789", "0.00000000000000");
        assert_mul(
            "-123456789.987654321",
            "987654321.123456789",
            "-121932632103337905.662094193112635269",
        );
        assert_mul("-123456789.987654321", "00000.00000", "0.00000000000000");
        assert_mul(
            "-123456789.987654321",
            "-987654321.123456789",
            "121932632103337905.662094193112635269",
        );
    }

    fn assert_div(val1: &str, val2: &str, expected: &str) {
        let var1 = val1.parse::<NumericVar>().unwrap();
        let var2 = val2.parse::<NumericVar>().unwrap();

        let result1 = &var1 / &var2;
        assert_eq!(result1.to_string(), expected);

        let mut result2 = var1.clone();
        result2 /= &var2;
        assert_eq!(result2.to_string(), expected);
    }

    #[test]
    fn div() {
        assert_div("NaN", "10000.00001", "NaN");
        assert_div("NaN", "00000.00000", "NaN");
        assert_div("NaN", "-10000.00001", "NaN");
        assert_div("10000.00001", "NaN", "NaN");
        assert_div("00000.00000", "NaN", "NaN");
        assert_div("-10000.00001", "NaN", "NaN");
        assert_div("NaN", "NaN", "NaN");
        assert_div(
            "0.000000001",
            "100000000",
            "0.000000000000000010000000000000000000",
        );
        assert_div("100000000", "0.000000001", "100000000000000000.000000000");
        assert_div(
            "123456789.987654321",
            "123456789.987654321",
            "1.00000000000000000000",
        );
        assert_div(
            "987654321.123456789",
            "987654321.123456789",
            "1.00000000000000000000",
        );
        assert_div(
            "123456789.987654321",
            "987654321.123456789",
            "0.12499999984531250018",
        );
        assert_div(
            "987654321.123456789",
            "123456789.987654321",
            "8.0000000099000000",
        );
        assert_div(
            "00000.00000",
            "123456789.987654321",
            "0.0000000000000000000000000000",
        );
        assert_div(
            "123456789.987654321",
            "-987654321.123456789",
            "-0.12499999984531250018",
        );
        assert_div(
            "-987654321.123456789",
            "123456789.987654321",
            "-8.0000000099000000",
        );
        assert_div(
            "00000.00000",
            "987654321.123456789",
            "0.0000000000000000000000000000",
        );
        assert_div(
            "00000.00000",
            "-987654321.123456789",
            "0.0000000000000000000000000000",
        );
        assert_div(
            "-123456789.987654321",
            "987654321.123456789",
            "-0.12499999984531250018",
        );
        assert_div(
            "987654321.123456789",
            "-123456789.987654321",
            "-8.0000000099000000",
        );
        assert_div(
            "00000.00000",
            "-123456789.987654321",
            "0.0000000000000000000000000000",
        );
        assert_div(
            "-123456789.987654321",
            "-987654321.123456789",
            "0.12499999984531250018",
        );
        assert_div(
            "-987654321.123456789",
            "-123456789.987654321",
            "8.0000000099000000",
        );
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn div_by_zero() {
        assert_div("1", "0", "");
    }
}
