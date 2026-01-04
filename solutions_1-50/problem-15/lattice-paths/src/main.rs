fn main() {
    // After many attempts of trying to do fun binary things (with python) and finding out that it would take 90 minutes to solve the problem,
    // I finally came to the conclusion of combinations...
    // I'm mad that it took me so long to figure it out because it was in the back of my head the whole time, and it's such a simple question
    // that just slipped my mind!
    // -- I guess that's why I am doing these problems!!!
    // Anyway, here is the solution.

    println!("{}", smart_ncr(40, 20));
}

fn smart_ncr(n: u128, k: u128) -> u128 {
    let mut r: u128 = 1;
    let k = if k > n / 2 { n - k } else { k };
    for i in 1..=k {
        // 40 * 39 * ... * 21
        // ------------------
        // 20 * 19 * ... *  1
        r *= n - i + 1;
        r /= i;
    }
    return r;
}
