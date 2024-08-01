#![feature(test)]

use fractions::Fraction;

use num_bigfloat::BigFloat;

use rug::Float;

fn calc_pi_fractions(iterations: u32) -> Fraction {
    let mut pi = Fraction::new(3, 1, false);
    let mut negative = false;

    for i in (2..iterations).step_by(2) {
        let term = Fraction::new(4, i * (i + 1) * (i + 2), negative);
        pi += term;

        negative = !negative;
    }

    return pi;
}

fn calc_pi_bigfloat(iterations: u32) -> BigFloat {
    let four: BigFloat = 4.into();
    let mut pi: BigFloat = 3.into();
    let mut sign: BigFloat = 1.into();

    for i in (2..iterations).step_by(2) {
        let term = four / BigFloat::from(i * (i + 1) * (i + 2));
        pi += sign * term;
        sign = -sign;
    }

    return pi;
}

fn calc_pi_rug(iterations: u32) -> Float {
    let four: Float = Float::with_val(53, 4);
    let mut pi: Float = Float::with_val(53, 3);
    let mut sign: Float = Float::with_val(53, 1);

    for i in (2..iterations).step_by(2) {
        let term = four.clone() / Float::with_val(53, i * (i + 1) * (i + 2));
        pi += sign.clone() * term;
        sign = -sign;
    }

    return pi;
}

fn main() {
    println!("Run `cargo bench` to run the benchmarks.");
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use std::hint::black_box;

    use test::Bencher;

    #[test]
    fn test_calc_pi_equal() {
        let pi_fractions = calc_pi_fractions(20);
        let pi_bigfloat = calc_pi_bigfloat(20);
        let pi_rug = calc_pi_rug(20);

        assert_eq!(pi_fractions.to_decimal_string(16), pi_rug.to_string());
        assert_eq!(pi_fractions.to_decimal_string(39), pi_bigfloat.to_string());
    }

    #[bench]
    fn bench_calc_pi_fractions(b: &mut Bencher) {
        b.iter(|| {
            black_box(calc_pi_fractions(black_box(20)));
        });
    }

    #[bench]
    fn bench_calc_pi_bigfloat(b: &mut Bencher) {
        b.iter(|| {
            black_box(calc_pi_bigfloat(black_box(20)));
        });
    }

    #[bench]
    fn bench_calc_pi_rug(b: &mut Bencher) {
        b.iter(|| {
            black_box(calc_pi_rug(black_box(20)));
        });
    }
}
