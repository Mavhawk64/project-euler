fn main() {
    let mut i: u64 = 286;
    loop {
        let j = ((12 * i * i + 12 * i + 1).isqrt() + 1) / 6;
        let k = ((12 * j * j - 4 * j + 1).isqrt() + 1) / 4;
        if t(i) == p(j) && p(j) == h(k) {
            println!("{}", t(i));
            break;
        }
        i += 1;
    }
}

fn t(n: u64) -> u64 {
    n * (n + 1) / 2
}
fn p(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}
fn h(n: u64) -> u64 {
    n * (2 * n - 1)
}
