fn main() {
    let mut s: u32 = 0;
    for a in 1..10 {
        s += count(a);
    }
    println!("{}", s);
}

// with a little bit of math, we can find:
// 10^{n-1}\leq a^n is our bound
// 1-1/n\leq\log_{10}a
// Max: a = 9:
// n\approx 21.###
// therefore, n = 21 is our upper bound.
// we can generate our counts with that formula above...
// n\leq\frac{1}{1-\log_{10}a}
// count(a) = \floor{\frac{1}{1-\log_{10}a}}
fn count(a: u32) -> u32 {
    (1f32 / (1f32 - (a as f32).log10())) as u32
}
