use num_bigint::BigUint;

fn main() {
    let mut max: u16 = 0;
    for a in 1u32..100 {
        for b in 1u32..100 {
            let base = BigUint::from(a);
            let num: u16 = base
                .pow(b)
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u16)
                .sum();
            if max < num {
                max = num;
            }
        }
    }
    println!("{}", max);
}

// max([sum([int(i) for i in list(str(a**b))]) for a in range(0,100) for b in range(0,100)])
