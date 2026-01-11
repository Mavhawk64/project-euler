fn main() {
    let mut sum: u32 = 0;
    let mut i: u32 = 0;
    let mut f: u32 = fib(i);
    while f < 4000000 {
        println!("{} {}", i, f);
        if f.is_multiple_of(2) {
            sum += f;
        }
        i += 1;
        f = fib(i);
    }
    println!("{}", sum);
}

fn fib(n: u32) -> u32 {
    if n <= 2 {
        return (n % 2 + n) / 2; // 0->0, 1->1, 2->1
    }
    fib(n - 1) + fib(n - 2)
}
