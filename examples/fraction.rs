use fractions::Fraction;

fn main() {
    let frac1 = Fraction::new(28, 4);
    let frac2 = Fraction::new(20, 24);

    println!("frac1: {}", frac1);
    println!("frac2: {}", frac2);

    let sum = frac1 + frac2;
    println!("Sum: {}", sum);

    let difference = frac1 - frac2;
    println!("Difference: {}", difference);

    let product = frac1 * frac2;
    println!("Product: {}", product);

    let quotient = frac1 / frac2;
    println!("Quotient: {}", quotient);

    let remainder = frac1 % frac2;
    println!("Remainder: {}", remainder);

    let negation = -frac1;
    println!("Negation of frac1: {}", negation);

    let simplified = frac1.simplify();
    println!("Simplified frac1: {}", simplified);
}
