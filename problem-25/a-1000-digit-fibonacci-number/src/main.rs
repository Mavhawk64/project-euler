use num_bigint::BigInt;
use num_traits::One; // Provides BigInt::one()
use num_traits::Zero; // Provides BigInt::zero()

fn main() {
    let mut i: u128 = 2;
    let mut fib_nums: Vec<BigInt> = Vec::new();
    fib_nums.push(BigInt::zero());
    fib_nums.push(BigInt::one());
    while fib_nums[fib_nums.len() - 1].to_string().len() < 1000 {
        fib_nums.push(&fib_nums[fib_nums.len() - 1] + &fib_nums[fib_nums.len() - 2]);
        i += 1;
    }
    println!("{}", i - 1);
}
