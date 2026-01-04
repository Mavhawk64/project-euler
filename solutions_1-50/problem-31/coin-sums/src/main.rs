use std::time::Instant;

fn main() {
    // 1. Time "My Way" (Brute Force)
    let start = Instant::now();
    let res1 = my_way();
    let duration1 = start.elapsed();
    println!("My Way: Result = {}, Time = {:?}", res1, duration1);

    // 2. Time "Gemini Optimization" (Pruned Loops)
    let start = Instant::now();
    let res2 = gemini_optimization();
    let duration2 = start.elapsed();
    println!(
        "Gemini Optimization: Result = {}, Time = {:?}",
        res2, duration2
    );

    // 3. Time Dynamic Programming (True Optimization)
    let start = Instant::now();
    let res3 = dynamic_programming();
    let duration3 = start.elapsed();
    println!(
        "Dynamic Programming: Result = {}, Time = {:?}",
        res3, duration3
    );
}

fn my_way() -> u32 {
    let mut s: u32 = 1;
    for a in 0..201 {
        for b in 0..101 {
            for c in 0..41 {
                for d in 0..21 {
                    for e in 0..11 {
                        for f in 0..5 {
                            for g in 0..3 {
                                if 200 == a + 2 * b + 5 * c + 10 * d + 20 * e + 50 * f + 100 * g {
                                    s += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    s
}

fn gemini_optimization() -> u32 {
    let mut count = 0;
    let target = 200;
    for a in (0..=target).step_by(200) {
        for b in (0..=(target - a)).step_by(100) {
            for c in (0..=(target - a - b)).step_by(50) {
                for d in (0..=(target - a - b - c)).step_by(20) {
                    for e in (0..=(target - a - b - c - d)).step_by(10) {
                        for f in (0..=(target - a - b - c - d - e)).step_by(5) {
                            for _g in (0..=(target - a - b - c - d - e - f)).step_by(2) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

fn dynamic_programming() -> u32 {
    let target = 200;
    let coins = [1, 2, 5, 10, 20, 50, 100, 200];
    let mut ways = vec![0; target + 1];
    ways[0] = 1;
    for &coin in &coins {
        for i in coin..=target {
            ways[i] += ways[i - coin];
        }
    }
    ways[target]
}
