#![cfg_attr(not(feature = "std"), no_std)]

use core::convert::From;
use core::fmt::Display;
use core::ops::{
    Add, AddAssign, BitOr, BitOrAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign,
    Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use core::u128;

pub trait Fractional:
    From<u8>
    + Copy
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + Not<Output = Self>
    + BitOr<Output = Self>
    + BitOrAssign
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + Display
{
    fn trailing_zeros(self) -> u32;

    #[cfg(feature = "std")]
    fn to_string(&self) -> String;
}

macro_rules! impl_fractional {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {
            fn trailing_zeros(self) -> u32 {
                self.trailing_zeros()
            }

            #[cfg(feature = "std")]
            fn to_string(&self) -> String {
                ToString::to_string(&self)
            }
        }
    )*)
}

impl_fractional!(Fractional for u8 u16 u32 u64 u128 usize);

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
pub struct Fraction<T>
where
    T: Fractional,
{
    pub numerator: T,
    pub denominator: T,
    pub sign: FractionSign,
}

impl<T> Fraction<T>
where
    T: Fractional,
{
    pub fn new(numerator: T, denominator: T, sign: FractionSign) -> Self {
        Fraction {
            numerator,
            denominator,
            sign,
        }
        .simplify()
    }

    pub fn simplify(self) -> Fraction<T> {
        let gcd = Self::gcd(self.numerator, self.denominator);
        Fraction {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
            sign: self.sign,
        }
    }

    fn gcd(mut a: T, mut b: T) -> T {
        if a == T::from(0) || b == T::from(0) {
            return a | b;
        }

        let shift = (a | b).trailing_zeros();

        a >>= a.trailing_zeros() as usize;

        while b != T::from(0) {
            b >>= b.trailing_zeros() as usize;

            if a > b {
                (a, b) = (b, a);
            }

            b -= a;
        }

        a << shift as usize
    }

    fn lcm(a: T, b: T) -> T {
        let gcd = Self::gcd(a, b);
        a * (b / gcd)
    }

    #[cfg(feature = "std")]
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
            numerator *= T::from(10);
            let mut digit = numerator / denominator;
            if i == precision - 1 {
                if digit >= T::from(5) {
                    digit += T::from(1);
                } else {
                    digit = T::from(0);
                }
            }
            result.push_str(&(digit).to_string());
            numerator %= denominator;
        }
        result
    }
}

impl<T> Add for Fraction<T>
where
    T: Fractional,
{
    type Output = Fraction<T>;

    fn add(self, other: Fraction<T>) -> Fraction<T> {
        if self.sign != other.sign {
            return self - -other;
        };

        if self.denominator == other.denominator {
            let numerator = self.numerator + other.numerator;
            return Fraction::new(numerator, self.denominator, FractionSign::Positive);
        }

        let lcm = Fraction::lcm(self.denominator, other.denominator);
        let numerator =
            self.numerator * (lcm / self.denominator) + other.numerator * (lcm / other.denominator);
        Fraction::new(numerator, lcm, FractionSign::Positive)
    }
}

impl<T> AddAssign for Fraction<T>
where
    T: Fractional,
{
    fn add_assign(&mut self, other: Fraction<T>) {
        *self = *self + other;
    }
}

impl<T> Sub for Fraction<T>
where
    T: Fractional,
{
    type Output = Fraction<T>;

    fn sub(self, other: Fraction<T>) -> Fraction<T> {
        if self.sign != other.sign {
            return self + -other;
        };

        if self.denominator == other.denominator {
            let numerator = if self.numerator > other.numerator {
                self.numerator - other.numerator
            } else {
                other.numerator - self.numerator
            };
            return Fraction::new(numerator, self.denominator, FractionSign::Positive);
        }

        let lcm = Fraction::lcm(self.denominator, other.denominator);
        let numerator1 = self.numerator * (lcm / self.denominator);
        let numerator2 = other.numerator * (lcm / other.denominator);

        let numerator = if numerator1 > numerator2 {
            numerator1 - numerator2
        } else {
            numerator2 - numerator1
        };

        Fraction::new(numerator, lcm, FractionSign::Positive)
    }
}

impl<T> SubAssign for Fraction<T>
where
    T: Fractional,
{
    fn sub_assign(&mut self, other: Fraction<T>) {
        *self = *self - other;
    }
}

impl<T> Mul for Fraction<T>
where
    T: Fractional,
{
    type Output = Fraction<T>;

    fn mul(self, other: Fraction<T>) -> Fraction<T> {
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

impl<T> MulAssign for Fraction<T>
where
    T: Fractional,
{
    fn mul_assign(&mut self, other: Fraction<T>) {
        *self = *self * other;
    }
}

impl<T> Div for Fraction<T>
where
    T: Fractional,
{
    type Output = Fraction<T>;

    fn div(self, other: Fraction<T>) -> Fraction<T> {
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

impl<T> DivAssign for Fraction<T>
where
    T: Fractional,
{
    fn div_assign(&mut self, other: Fraction<T>) {
        *self = *self / other;
    }
}

impl<T> Neg for Fraction<T>
where
    T: Fractional,
{
    type Output = Fraction<T>;

    fn neg(self) -> Fraction<T> {
        Fraction::new(self.numerator, self.denominator, !self.sign)
    }
}

impl<T> Rem for Fraction<T>
where
    T: Fractional,
{
    type Output = Fraction<T>;

    fn rem(self, other: Fraction<T>) -> Fraction<T> {
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

impl<T> RemAssign for Fraction<T>
where
    T: Fractional,
{
    fn rem_assign(&mut self, other: Fraction<T>) {
        *self = *self % other;
    }
}

impl<T> PartialEq for Fraction<T>
where
    T: Fractional,
{
    fn eq(&self, other: &Self) -> bool {
        self.numerator * other.denominator == other.numerator * self.denominator
    }
}

impl<T> Display for Fraction<T>
where
    T: Fractional,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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
            impl<T> From<$t> for Fraction<T>
            where
                T: Fractional + From<$t>,
            {
                fn from(n: $t) -> Self {
                    #[allow(unused_comparisons)]
                    Fraction::new(T::from(n), T::from(1u8), if n < 0 { FractionSign::Negative } else { FractionSign::Positive })
                }
            }
        )*
    };
}

// Implement conversion from Fraction to float
macro_rules! impl_into {
    ($($t:ty),*) => {
        $(
            impl<T> From<Fraction<T>> for $t
            where
                T: Fractional, $t: From<T>,
            {
                fn from(f: Fraction<T>) -> Self {
                    let sign = if f.sign == FractionSign::Negative { -1.0 } else { 1.0 };
                    sign * (<$t>::from(f.numerator)) / (<$t>::from(f.denominator))
                }
            }
        )*
    };
}

// Define a macro to implement operations for various types
macro_rules! impl_operation {
    ($op_trait:ident, $op_fn:ident, $($t:ty),*) => {
        $(
            impl<T> $op_trait<Fraction<T>> for $t
            where
                T: Fractional + From<$t>,
            {
                type Output = Fraction<T>;

                fn $op_fn(self, other: Fraction<T>) -> Fraction<T> {
                    #[allow(unused_comparisons)]
                    Fraction::new(T::from(self), T::from(1u8), if self < 0 { FractionSign::Negative } else { FractionSign::Positive }).$op_fn(other)
                }
            }

            impl<T> $op_trait<$t> for Fraction<T>
            where
                T: Fractional + From<$t>,
            {
                type Output = Fraction<T>;

                fn $op_fn(self, other: $t) -> Fraction<T> {
                    #[allow(unused_comparisons)]
                    self.$op_fn(Fraction::new(T::from(other), T::from(1u8), if other < 0 { FractionSign::Negative } else { FractionSign::Positive }))
                }
            }
        )*
    };
}

// Define a macro to implement assignment operations
macro_rules! impl_assign {
    ($op_trait:ident, $op_fn:ident, $($t:ty),*) => {
        $(
            impl<T> $op_trait<$t> for Fraction<T>
            where
                T: Fractional + From<$t>,
            {
                fn $op_fn(&mut self, other: $t) {
                    #[allow(unused_comparisons)]
                    self.$op_fn(Fraction::new(T::from(other), T::from(1u8), if other < 0 { FractionSign::Negative } else { FractionSign::Positive }));
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
        let f: Fraction<u8> = Fraction::new(4, 6, FractionSign::Positive);
        assert_eq!(f.numerator, 2);
        assert_eq!(f.denominator, 3);
        assert_eq!(f.sign, FractionSign::Positive);

        let f: Fraction<u16> = Fraction::new(123, 456, FractionSign::Positive);
        assert_eq!(f.numerator, 41);
        assert_eq!(f.denominator, 152);
        assert_eq!(f.sign, FractionSign::Positive);
    }

    #[test]
    fn test_add() {
        let f1: Fraction<u16> = Fraction::new(1, 2, FractionSign::Positive);
        let f2: Fraction<u16> = Fraction::new(1, 3, FractionSign::Positive);
        let f3: Fraction<u16> = Fraction::new(1, 2, FractionSign::Negative);
        let f4: Fraction<u16> = Fraction::new(10, 2, FractionSign::Positive);

        let mut f = f1 + f2;

        assert_eq!(f.numerator, 5);
        assert_eq!(f.denominator, 6);
        assert_eq!(f.sign, FractionSign::Positive);

        f += f3;

        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 3);
        assert_eq!(f.sign, FractionSign::Positive);

        f += f4;

        assert_eq!(f.numerator, 16);
        assert_eq!(f.denominator, 3);
        assert_eq!(f.sign, FractionSign::Positive);
    }

    #[test]
    fn test_sub() {
        let f1: Fraction<u8> = Fraction::new(1, 2, FractionSign::Positive);
        let f2: Fraction<u8> = Fraction::new(1, 3, FractionSign::Positive);
        let f3: Fraction<u8> = Fraction::new(1, 2, FractionSign::Negative);
        let f4: Fraction<u8> = Fraction::new(10, 2, FractionSign::Positive);

        let f = f1 - f2;

        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 6);
        assert_eq!(f.sign, FractionSign::Positive);

        let f = f - f3;

        assert_eq!(f.numerator, 2);
        assert_eq!(f.denominator, 3);
        assert_eq!(f.sign, FractionSign::Positive);

        let f = f - f4;

        assert_eq!(f.numerator, 13);
        assert_eq!(f.denominator, 3);
        assert_eq!(f.sign, FractionSign::Positive);
    }

    #[test]
    fn test_mul() {
        let f1: Fraction<u8> = Fraction::new(1, 2, FractionSign::Positive);
        let f2: Fraction<u8> = Fraction::new(1, 3, FractionSign::Positive);
        let f3: Fraction<u8> = Fraction::new(1, 2, FractionSign::Negative);
        let f4: Fraction<u8> = Fraction::new(10, 2, FractionSign::Positive);

        let f = f1 * f2;
        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 6);
        assert_eq!(f.sign, FractionSign::Positive);

        let f = f * f3;

        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 12);
        assert_eq!(f.sign, FractionSign::Negative);

        let f = f * f4;

        assert_eq!(f.numerator, 5);
        assert_eq!(f.denominator, 12);
        assert_eq!(f.sign, FractionSign::Negative);
    }

    #[test]
    fn test_div() {
        let f1: Fraction<u8> = Fraction::new(1, 2, FractionSign::Positive);
        let f2: Fraction<u8> = Fraction::new(1, 3, FractionSign::Positive);
        let f3: Fraction<u8> = Fraction::new(1, 2, FractionSign::Negative);
        let f4: Fraction<u8> = Fraction::new(10, 2, FractionSign::Positive);

        let f = f1 / f2;
        assert_eq!(f.numerator, 3);
        assert_eq!(f.denominator, 2);
        assert_eq!(f.sign, FractionSign::Positive);

        let f = f / f3;

        assert_eq!(f.numerator, 3);
        assert_eq!(f.denominator, 1);
        assert_eq!(f.sign, FractionSign::Negative);

        let f = f / f4;

        assert_eq!(f.numerator, 3);
        assert_eq!(f.denominator, 5);
        assert_eq!(f.sign, FractionSign::Negative);
    }

    #[test]
    fn test_neg() {
        let f: Fraction<u8> = Fraction::new(1, 2, FractionSign::Positive);
        let f = -f;
        assert_eq!(-0.5, f.into());

        let f: Fraction<u8> = Fraction::new(1, 2, FractionSign::Negative);
        let f = -f;
        assert_eq!(0.5, f.into());
    }

    #[test]
    fn test_rem() {
        let f1: Fraction<u8> = Fraction::new(1, 2, FractionSign::Positive);
        let f2: Fraction<u8> = Fraction::new(1, 3, FractionSign::Positive);
        let f = f1 % f2;
        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 6);
        assert_eq!(f.sign, FractionSign::Positive);
    }

    #[test]
    fn test_eq() {
        let f1: Fraction<u8> = Fraction::new(1, 2, FractionSign::Positive);
        let f2: Fraction<u8> = Fraction::new(1, 3, FractionSign::Positive);
        assert_ne!(f1, f2);

        let f1: Fraction<u8> = Fraction::new(1, 2, FractionSign::Positive);
        let f2: Fraction<u8> = Fraction::new(2, 4, FractionSign::Positive);
        assert_eq!(f1, f2);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_display() {
        let f: Fraction<u8> = Fraction::new(1, 2, FractionSign::Positive);
        assert_eq!(format!("{}", f), "1/2");

        let f: Fraction<u8> = Fraction::new(1, 2, FractionSign::Negative);
        assert_eq!(format!("{}", f), "-1/2");
    }
}
