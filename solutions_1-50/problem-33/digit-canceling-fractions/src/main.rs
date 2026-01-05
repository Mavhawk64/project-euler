use num_rational::Ratio;
fn main() {
    // Naive method: (10a + b) / (10c + d)
    let mut ratios: Vec<Ratio<u8>> = Vec::new();
    for a in 0..10 {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    if 0 == 10 * c + d
                        || 0 == 10 * a + b
                        || 10 * a + b >= 10 * c + d
                        || 10 * a + b < 10
                        || 10 * c + d < 10
                        || 0 == a
                        || 0 == b
                        || 0 == c
                        || 0 == d
                    {
                        continue;
                    }
                    let r: Ratio<u8> = Ratio::new(10 * a + b, 10 * c + d);
                    let red: Option<Ratio<u8>> = if a == c {
                        Some(Ratio::new(b, d))
                    } else if b == d {
                        Some(Ratio::new(a, c))
                    } else if a == d {
                        Some(Ratio::new(b, c))
                    } else if b == c {
                        Some(Ratio::new(a, d))
                    } else {
                        None
                    };
                    if red == Some(r) {
                        println!("{}/{} is special!", 10 * a + b, 10 * c + d);
                        ratios.push(r);
                    }
                }
            }
        }
    }
    println!("{:#?}", ratios);

    let prod: Ratio<u8> = ratios.iter().product();
    println!("Product Ratio:\n{:?}\nSolution:\n{}", prod, prod.denom());
}
