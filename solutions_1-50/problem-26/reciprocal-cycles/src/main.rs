use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{pow, One};
use std::str::FromStr;

// We need to find the biggest pattern from 1/1 -> 1/1000 in the decimals
// I know a cute trick --> see if it matches to x/9, xy/99, xyz/999 ...

fn main() {
    let mut m: u16 = 0;
    let mut x: usize = 0;
    for i in 1..1000 {
        let o = find_pattern_inverse(i as u16);
        println!("len(1/{}) = {}", i, o);
        if o > m {
            m = o;
            x = i;
        }
    }
    println!("Longest is {} with a length of {}!", x, m);
}

fn find_pattern_inverse(n: u16) -> u16 {
    let frac: Ratio<BigInt> = Ratio::new(BigInt::one(), BigInt::from(n));
    let mut cnt: u32 = 0;
    while cnt < 100 {
        for l in 1..n {
            let x =
                frac.clone() * BigInt::from_str(&get_nines(l)).unwrap() * BigInt::from(10).pow(cnt);
            if x.denom() == &BigInt::one() {
                println!("{} is of length {}!", n, l);
                return l;
            }
        }
        cnt += 1;
    }

    return 0;
}

fn get_nines(n: u16) -> String {
    let mut s: String = "".to_string();
    for _ in 0..n {
        s += "9";
    }
    return s;
}

fn ratio_to_bigdecimal(ratio: Ratio<BigInt>, precision: i64) -> BigDecimal {
    let num = BigDecimal::from(ratio.numer().clone());
    let den = BigDecimal::from(ratio.denom().clone());

    // Use with_scale to set decimal places
    (num / den).with_scale(precision)
}
