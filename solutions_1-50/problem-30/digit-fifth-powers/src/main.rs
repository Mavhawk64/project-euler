const FIFTH_POWERS: [u32; 10] = [0, 1, 32, 243, 1024, 3125, 7776, 16807, 32768, 59049];
fn main() {
    let mut s: u32 = 0;
    // start at 2 because 1 doesn't count.
    for i in 2..1_000_000 {
        if is_fifth_power_sum(i as u32) {
            println!("Found a Digit Fifth Power! {}", i);
            s += i as u32;
        }
    }
    println!("The sum of these numbers is {}", s);
}

fn is_fifth_power_sum(d: u32) -> bool {
    return d
        == d.to_string()
            .chars()
            .map(|i| FIFTH_POWERS[i.to_digit(10).unwrap() as usize])
            .sum();
}
