fn main() {
    let mut c: usize = 0;
    for n in 1..101 {
        for r in 1..=n / 2 {
            if ncr(n, r) > 1_000_000 {
                c += 1
                    * (if n.is_multiple_of(2) && r == n / 2 {
                        1
                    } else {
                        2
                    });
            }
        }
    }
    println!("{}", c);
}

fn ncr(n: u128, r: u128) -> u128 {
    let r = r.min(n - r);
    let mut result = 1u128;

    for i in 0..r {
        result = result * (n - i) / (i + 1);
    }
    result
}
