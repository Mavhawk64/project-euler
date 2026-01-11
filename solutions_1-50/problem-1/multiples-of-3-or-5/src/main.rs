fn main() {
    let mut sum: u32 = 0;
    for i in 0..1000 {
        sum += if i % 3 == 0 || i % 5 == 0 { i } else { 0 };
    }
    println!("{}", sum);
}
