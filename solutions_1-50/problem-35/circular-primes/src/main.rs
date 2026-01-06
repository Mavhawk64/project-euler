use primes::is_prime;

fn main() {
    let mut cpv: Vec<u32> = Vec::new();
    for i in 2..1_000_000 {
        if is_circularly_prime(i) {
            cpv.push(i as u32);
        }
    }
    println!("len({:?}) = {}", cpv, cpv.len());
}

fn is_circularly_prime(x: u32) -> bool {
    let s = x.to_string();
    let mut c: Vec<char> = s.chars().collect();
    for _ in 0..c.len() {
        let rotated_str: String = c.iter().collect();
        let rotated_num: u32 = rotated_str.parse().unwrap();
        if !is_prime(rotated_num as u64) {
            return false;
        }
        let first = c.remove(0);
        c.push(first);
    }
    return true;
}
