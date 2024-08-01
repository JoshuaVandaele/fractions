use fractions::{Fraction, FractionSign};

// Calculate pi using the Nilakantha series
fn main() {
    let mut pi = Fraction::new(3, 1, FractionSign::Positive);
    let mut sign = FractionSign::Positive;

    for i in (2..20).step_by(2) {
        let term = Fraction::new(4, i * (i + 1) * (i + 2), sign);
        pi += term;
        sign = !sign;
    }

    println!(
        "According to ten terms of the Nilakantha series, pi is approximately {pi}, or {}",
        pi.to_decimal_string(16)
    );
}
