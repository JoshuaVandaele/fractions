#![feature(test)]

use fractions::Fraction;

use num_bigfloat::BigFloat;

use rug::Float;

fn calc_pi_fractions(iterations: u32) {
    let mut pi = Fraction::new(3, 1);
    let mut sign = 1;

    for i in (2..iterations).step_by(2) {
        let term = Fraction::new(4, i * (i + 1) * (i + 2));
        pi += sign * term;
        sign *= -1;
    }
}

fn calc_pi_bigfloat(iterations: u32) {
    let four: BigFloat = 4.into();
    let mut pi: BigFloat = 3.into();
    let mut sign: BigFloat = 1.into();

    for i in (2..iterations).step_by(2) {
        let term = four / BigFloat::from(i * (i + 1) * (i + 2));
        pi += sign * term;
        sign = -sign;
    }
}

fn calc_pi_rug(iterations: u32) {
    let four: Float = Float::with_val(53, 4);
    let mut pi: Float = Float::with_val(53, 3);
    let mut sign: Float = Float::with_val(53, 1);

    for i in (2..iterations).step_by(2) {
        let term = four.clone() / Float::with_val(53, i * (i + 1) * (i + 2));
        pi += sign.clone() * term;
        sign = -sign;
    }
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
