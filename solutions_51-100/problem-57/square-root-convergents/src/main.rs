use num_bigint::BigUint;
use num_rational::Ratio;
use num_traits::One;

fn main() {
    println!("{}", sqrt2(1000));
}

fn sqrt2(depth: usize) -> usize {
    // 1 + 1/2 -> depth => 1
    // 1 +  1/(1 + (1 + 1/2)) -> depth - 1 => 2 ...
    let mut ret: usize = 0;
    let mut rat: Ratio<BigUint> = Ratio::new(BigUint::from(3u32), BigUint::from(2u32));
    println!("{} / {}", rat.numer(), rat.denom());
    for _ in 2..=depth {
        let one: Ratio<BigUint> = Ratio::one();
        rat = one.clone() + one.clone() / (one.clone() + rat);
        println!("\n{} / {}", rat.numer(), rat.denom());
        if rat.numer().to_string().len() > rat.denom().to_string().len() {
            ret += 1;
        }
    }
    ret
}
