fn main() {
    println!(
        "{}",
        get_nth_digit(1)
            * get_nth_digit(10)
            * get_nth_digit(100)
            * get_nth_digit(1_000)
            * get_nth_digit(10_000)
            * get_nth_digit(100_000)
            * get_nth_digit(1_000_000)
    ); // Actually, this could have ended up as an overflow if these were bigger numbers XD
}

// https://www.desmos.com/calculator/sgzlrj5vbq -- corresponding Desmos page.

fn get_nth_digit(d: u32) -> u8 {
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    while b < d {
        a += 1;
        b += a.ilog(10) + 1;
    }
    // a is our number, b - d -> index from the right of a.
    ((a / 10u32.pow(b - d)) % 10) as u8
}
