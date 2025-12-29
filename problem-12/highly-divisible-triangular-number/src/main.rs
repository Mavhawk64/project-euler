fn main() {
    let mut iter: u128 = 1;
    let mut i: u128 = 1;
    let mut f: Vec<u128> = get_factors(i);
    let mut bf: usize = 1;
    while f.len() < 500 {
        iter += 1;
        i += iter;
        f = get_factors(i);
        if f.len() > bf {
            bf = f.len();
            println!("New biggest: {} with {} factors: {:?}", i, f.len(), f);
        }
    }
    println!(
        "{} has {} factors.\nHere they are listed:\n{:#?}",
        i,
        f.len(),
        f
    );
}

fn get_factors(n: u128) -> Vec<u128> {
    let mut v: Vec<u128> = [].to_vec();
    for i in 1..n.isqrt() + 1 {
        if n % i == 0 {
            v.push(i);
            v.push(n / i);
        }
    }
    v.sort();
    return v;
}
