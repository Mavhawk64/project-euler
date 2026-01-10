use num_bigint::BigInt;
fn main() {
    let mut s: BigInt = BigInt::ZERO;
    for i in 1..1001 {
        s += BigInt::pow(&BigInt::from(i), i);
    }
    // print the last 10 digits:
    let last_10_digits: BigInt = s % BigInt::from(10_000_000_000u64);
    println!("{}", last_10_digits);
}
