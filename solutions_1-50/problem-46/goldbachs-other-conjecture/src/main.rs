use primes::is_prime;
fn main() {
    let mut n: u64 = 4; // 9
    loop {
        let lhs = 2 * n + 1;
        println!("2*{} + 1 = {}", n, lhs);
        n += 1;
        if is_prime(lhs) {
            continue;
        }
        if !find_a_p(lhs) {
            println!("{}", lhs);
            break;
        }
    }
}

fn find_a_p(oc: u64) -> bool {
    let mut p = 2;
    while p < oc - 2 {
        let mut p2 = p;
        while !is_prime(p2) {
            p2 += 1;
        }
        if p2 <= oc {
            for a in 1..((oc - p2) / 2).isqrt() + 1 {
                if p2 + 2 * a * a == oc {
                    println!("{} = {} + 2 x {} ** 2", oc, p2, a);
                    return true;
                }

                if p2 + 2 * a * a > oc {
                    break;
                }
            }
        }
        p += 1;
    }
    false
}
