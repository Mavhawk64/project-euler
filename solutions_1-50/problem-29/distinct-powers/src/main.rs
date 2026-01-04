use num_bigint::BigUint;
use num_traits::Pow;
use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant; // Standard timing library // Thanks to Gemini for adding timing.

fn main() {
    let min: i32 = 2;
    let max: i32 = 1000;

    let start: Instant = Instant::now(); // Start timer

    let mut ret: HashSet<BigUint> = HashSet::new();
    for a in min..=max {
        let base: BigUint = BigUint::from(a as u64);
        for b in min..=max {
            let val: BigUint = base.clone().pow(b as u32);
            ret.insert(val);
        }
    }

    let duration: Duration = start.elapsed(); // End timer

    println!("Result: {}", ret.len());
    println!("Time: {:?}", duration);
}
