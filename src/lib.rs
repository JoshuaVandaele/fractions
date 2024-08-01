use std::convert::From;
use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy)]
pub enum FractionSign {
    Negative,
    Positive,
}

impl Not for FractionSign {
    type Output = FractionSign;

    fn not(self) -> FractionSign {
        match self {
            FractionSign::Positive => FractionSign::Negative,
            FractionSign::Negative => FractionSign::Positive,
        }
    }
}

impl PartialEq for FractionSign {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FractionSign::Positive, FractionSign::Positive) => true,
            (FractionSign::Negative, FractionSign::Negative) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Fraction {
    pub numerator: u32,
    pub denominator: u32,
    pub sign: FractionSign,
}

impl Fraction {
    pub fn new(numerator: u32, denominator: u32, sign: FractionSign) -> Self {
        Fraction {
            numerator,
            denominator,
            sign,
        }
        .simplify()
    }

    pub fn simplify(self) -> Fraction {
        let gcd = Self::gcd(self.numerator, self.denominator);
        Fraction {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
            sign: self.sign,
        }
    }

    fn gcd(mut a: u32, mut b: u32) -> u32 {
        if a == 0 || b == 0 {
            return a | b;
        }

        let shift = (a | b).trailing_zeros();

        a >>= a.trailing_zeros();

        while b != 0 {
            b >>= b.trailing_zeros();

            if a > b {
                (a, b) = (b, a);
            }

            b -= a;
        }

        a << shift
    }

    // function to display the fraction in the form of a decimal with infinite precision
    pub fn to_decimal_string(&self, precision: usize) -> String {
        let mut result = String::new();
        let mut numerator = self.numerator;
        let denominator = self.denominator;

        if self.sign == FractionSign::Negative {
            result.push('-');
        }
        result.push_str(&(numerator / denominator).to_string());
        result.push('.');
        numerator %= denominator;
        for i in 0..precision {
            numerator *= 10;
            let mut digit = numerator / denominator;
            if i == precision - 1 {
                if digit >= 5 {
                    digit += 1;
                } else {
                    digit = 0;
                }
            }
            result.push_str(&(digit).to_string());
            numerator %= denominator;
        }
        result
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        let sign = if self.sign == other.sign {
            FractionSign::Positive
        } else {
            FractionSign::Negative
        };

        if sign == FractionSign::Negative {
            return self - -other;
        }

        let numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        let denominator = self.denominator * other.denominator;
        Fraction::new(numerator, denominator, sign)
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, other: Fraction) {
        *self = *self + other;
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, other: Fraction) -> Fraction {
        let sign = if self.sign == other.sign {
            FractionSign::Positive
        } else {
            FractionSign::Negative
        };

        if sign == FractionSign::Negative {
            return self + -other;
        }

        let numerator1 = self.numerator * other.denominator;
        let numerator2 = other.numerator * self.denominator;

        let numerator = if numerator1 > numerator2 {
            numerator1 - numerator2
        } else {
            numerator2 - numerator1
        };

        let denominator = self.denominator * other.denominator;
        Fraction::new(numerator, denominator, sign)
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, other: Fraction) {
        *self = *self - other;
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Fraction {
        let numerator = self.numerator * other.numerator;
        let denominator = self.denominator * other.denominator;
        let sign = if self.sign == other.sign {
            FractionSign::Positive
        } else {
            FractionSign::Negative
        };
        Fraction::new(numerator, denominator, sign)
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, other: Fraction) {
        *self = *self * other;
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, other: Fraction) -> Fraction {
        let numerator = self.numerator * other.denominator;
        let denominator = self.denominator * other.numerator;
        let sign = if self.sign == other.sign {
            FractionSign::Positive
        } else {
            FractionSign::Negative
        };
        Fraction::new(numerator, denominator, sign)
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, other: Fraction) {
        *self = *self / other;
    }
}

impl Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Fraction {
        Fraction::new(self.numerator, self.denominator, !self.sign)
    }
}

impl Rem for Fraction {
    type Output = Fraction;

    fn rem(self, other: Fraction) -> Fraction {
        let numerator = (self.numerator * other.denominator) % (other.numerator * self.denominator);
        let denominator = self.denominator * other.denominator;
        let sign = if self.sign == other.sign {
            FractionSign::Positive
        } else {
            FractionSign::Negative
        };
        Fraction::new(numerator, denominator, sign)
    }
}

impl RemAssign for Fraction {
    fn rem_assign(&mut self, other: Fraction) {
        *self = *self % other;
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.numerator * other.denominator == other.numerator * self.denominator
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}/{}",
            if self.sign == FractionSign::Negative {
                "-"
            } else {
                ""
            },
            self.numerator,
            self.denominator
        )
    }
}

// implement for all integer types using macro
macro_rules! impl_from {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Fraction {
                fn from(n: $t) -> Self {
                    #[allow(unused_comparisons)]
                    Fraction::new(n as u32, 1, if n < 0 { FractionSign::Negative } else { FractionSign::Positive })
                }
            }
        )*
    };
}

// Implement conversion from Fraction to float
macro_rules! impl_into {
    ($($t:ty),*) => {
        $(
            impl From<Fraction> for $t {
                fn from(f: Fraction) -> Self {
                    if f.sign == FractionSign::Negative {
                        return -(f.numerator as $t / f.denominator as $t);
                    }
                    f.numerator as $t / f.denominator as $t
                }
            }
        )*
    };
}

// Define a macro to implement operations for various types
macro_rules! impl_operation {
    ($op_trait:ident, $op_fn:ident, $($t:ty),*) => {
        $(
            impl $op_trait<Fraction> for $t {
                type Output = Fraction;

                fn $op_fn(self, other: Fraction) -> Fraction {
                    #[allow(unused_comparisons)]
                    Fraction::new(self as u32, 1, if self < 0 { FractionSign::Negative } else { FractionSign::Positive }).$op_fn(other)
                }
            }

            impl $op_trait<$t> for Fraction {
                type Output = Fraction;

                fn $op_fn(self, other: $t) -> Fraction {
                    #[allow(unused_comparisons)]
                    self.$op_fn(Fraction::new(other as u32, 1, if other < 0 { FractionSign::Negative } else { FractionSign::Positive }))
                }
            }
        )*
    };
}

// Define a macro to implement assignment operations
macro_rules! impl_assign {
    ($op_trait:ident, $op_fn:ident, $($t:ty),*) => {
        $(
            impl $op_trait<$t> for Fraction {
                fn $op_fn(&mut self, other: $t) {
                    #[allow(unused_comparisons)]
                    self.$op_fn(Fraction::new(other as u32, 1, if other < 0 { FractionSign::Negative } else { FractionSign::Positive }));
                }
            }
        )*
    };
}

impl_from!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_into!(f32, f64);

impl_operation!(Mul, mul, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_operation!(Add, add, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_operation!(Div, div, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_operation!(Sub, sub, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_operation!(Rem, rem, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

impl_assign!(AddAssign, add_assign, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_assign!(SubAssign, sub_assign, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_assign!(MulAssign, mul_assign, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_assign!(DivAssign, div_assign, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_assign!(RemAssign, rem_assign, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify() {
        let f = Fraction::new(4, 6, FractionSign::Positive);
        assert_eq!(f.numerator, 2);
        assert_eq!(f.denominator, 3);

        let f = Fraction::new(123, 456, FractionSign::Positive);
        assert_eq!(f.numerator, 41);
        assert_eq!(f.denominator, 152);
    }

    #[test]
    fn test_add() {
        let f1 = Fraction::new(1, 2, FractionSign::Positive);
        let f2 = Fraction::new(1, 3, FractionSign::Positive);
        let f3 = Fraction::new(1, 2, FractionSign::Negative);
        let f4 = Fraction::new(10, 2, FractionSign::Positive);

        let mut f = f1 + f2;

        assert_eq!(f.numerator, 5);
        assert_eq!(f.denominator, 6);

        f += f3;

        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 3);

        f += f4;

        assert_eq!(f.numerator, 16);
        assert_eq!(f.denominator, 3);
    }

    #[test]
    fn test_sub() {
        let f1 = Fraction::new(1, 2, FractionSign::Positive);
        let f2 = Fraction::new(1, 3, FractionSign::Positive);
        let f3 = Fraction::new(1, 2, FractionSign::Negative);
        let f4 = Fraction::new(10, 2, FractionSign::Positive);

        let f = f1 - f2;

        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 6);

        let f = f - f3;

        assert_eq!(f.numerator, 2);
        assert_eq!(f.denominator, 3);

        let f = f - f4;

        assert_eq!(f.numerator, 13);
        assert_eq!(f.denominator, 3);
    }

    #[test]
    fn test_mul() {
        let f1 = Fraction::new(1, 2, FractionSign::Positive);
        let f2 = Fraction::new(1, 3, FractionSign::Positive);
        let f3 = Fraction::new(1, 2, FractionSign::Negative);
        let f4 = Fraction::new(10, 2, FractionSign::Positive);

        let f = f1 * f2;
        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 6);

        let f = f * f3;

        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 12);

        let f = f * f4;

        assert_eq!(f.numerator, 5);
        assert_eq!(f.denominator, 12);
    }

    #[test]
    fn test_div() {
        let f1 = Fraction::new(1, 2, FractionSign::Positive);
        let f2 = Fraction::new(1, 3, FractionSign::Positive);
        let f3 = Fraction::new(1, 2, FractionSign::Negative);
        let f4 = Fraction::new(10, 2, FractionSign::Positive);

        let f = f1 / f2;
        assert_eq!(f.numerator, 3);
        assert_eq!(f.denominator, 2);

        let f = f / f3;

        assert_eq!(f.numerator, 3);
        assert_eq!(f.denominator, 1);

        let f = f / f4;

        assert_eq!(f.numerator, 3);
        assert_eq!(f.denominator, 5);
    }

    #[test]
    fn test_neg() {
        let f = Fraction::new(1, 2, FractionSign::Positive);
        let f = -f;
        assert_eq!(-0.5, f.into());

        let f = Fraction::new(1, 2, FractionSign::Negative);
        let f = -f;
        assert_eq!(0.5, f.into());
    }

    #[test]
    fn test_rem() {
        let f1 = Fraction::new(1, 2, FractionSign::Positive);
        let f2 = Fraction::new(1, 3, FractionSign::Positive);
        let f = f1 % f2;
        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 6);
    }

    #[test]
    fn test_eq() {
        let f1 = Fraction::new(1, 2, FractionSign::Positive);
        let f2 = Fraction::new(1, 3, FractionSign::Positive);
        assert_ne!(f1, f2);

        let f1 = Fraction::new(1, 2, FractionSign::Positive);
        let f2 = Fraction::new(2, 4, FractionSign::Positive);
        assert_eq!(f1, f2);
    }

    #[test]
    fn test_display() {
        let f = Fraction::new(1, 2, FractionSign::Positive);
        assert_eq!(format!("{}", f), "1/2");

        let f = Fraction::new(1, 2, FractionSign::Negative);
        assert_eq!(format!("{}", f), "-1/2");
    }
}
