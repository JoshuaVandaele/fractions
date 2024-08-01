use fractions::{Fraction, FractionSign};

// Calculate pi using the Nilakantha series
fn main() {
    let mut pi: Fraction<u128> = Fraction::new(3, 1, FractionSign::Positive);
    let mut sign = FractionSign::Positive;

    for i in (2..22).step_by(2) {
        let term = Fraction::new(4, i * (i + 1) * (i + 2), sign);
        pi += term;
        sign = !sign;

        #[cfg(feature = "std")]
        println!("{}: {pi} - {}", i / 2, pi.to_decimal_string(16));

        #[cfg(not(feature = "std"))]
        println!("{}: {}", i / 2, pi);
    }

    #[cfg(feature = "std")]
    println!(
        "According to ten terms of the Nilakantha series, pi is approximately {pi}, or {}",
        pi.to_decimal_string(16)
    );

    #[cfg(not(feature = "std"))]
    println!(
        "According to ten terms of the Nilakantha series, pi is approximately {}",
        pi
    );
}
