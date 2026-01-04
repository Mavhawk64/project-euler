fn main() {
    let mut s: u128 = 0;
    // precalculate abundants
    let mut ab: Vec<u8> = [].to_vec();
    for i in 0..28124 {
        ab.push(is_abundant(i) as u8);
    }
    for i in 0..28124 {
        if !is_double_abundant(i, &ab) {
            s += i as u128;
            println!(
                "{} is not double abundant! d({}) = {} \n{:#?}",
                i,
                i,
                d(i as u128),
                proper_divisors(i as u128)
            );
        }
    }
    println!("{}", s);
}

fn is_abundant(n: u128) -> bool {
    return d(n) > n;
}

fn is_double_abundant(n: usize, ab_map: &Vec<u8>) -> bool {
    for a in 12..=(n / 2) {
        let b = n - a;
        if ab_map[a] + ab_map[b] == 2 {
            return true;
        }
    }
    return false;
}

fn d(n: u128) -> u128 {
    return proper_divisors(n).iter().sum();
}

fn proper_divisors(n: u128) -> Vec<u128> {
    if n < 2 {
        return vec![];
    }
    let mut v: Vec<u128> = Vec::new();
    let limit = n.isqrt();
    for i in 1..=limit {
        if n % i == 0 {
            v.push(i);
            let counterpart = n / i;
            if counterpart != n && counterpart != i {
                v.push(counterpart);
            }
        }
    }
    return v;
}
