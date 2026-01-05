const FACTORIALS: [u32; 10] = [1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880];

fn main() {
    let mut ret: Vec<u32> = Vec::new();
    // naive way: a_bcd_efg
    for a in 0..10 {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    for e in 0..10 {
                        for f in 0..10 {
                            for g in 0..10 {
                                if a == 0 && b == 0 && c == 0 && d == 0 && e == 0 && f == 0 {
                                    continue; // not a sum
                                }
                                let num: u32 = (1_000_000 * a
                                    + 100_000 * b
                                    + 10_000 * c
                                    + 1_000 * d
                                    + 100 * e
                                    + 10 * f
                                    + g) as u32;
                                let mut fact_sum: u32 = 0;
                                let mut output: String = "".to_string();
                                if a == 0 && b == 0 && c == 0 && d == 0 && e == 0 && f != 0 {
                                    fact_sum = FACTORIALS[f] + FACTORIALS[g];
                                    output = format!("{}! + {}! = {}", f, g, fact_sum);
                                } else if a == 0 && b == 0 && c == 0 && d == 0 && e != 0 {
                                    fact_sum = FACTORIALS[e] + FACTORIALS[f] + FACTORIALS[g];
                                    output = format!("{}! + {}! + {}! = {}", e, f, g, fact_sum);
                                } else if a == 0 && b == 0 && c == 0 && d != 0 {
                                    fact_sum = FACTORIALS[d]
                                        + FACTORIALS[e]
                                        + FACTORIALS[f]
                                        + FACTORIALS[g];
                                    output =
                                        format!("{}! + {}! + {}! + {}! = {}", d, e, f, g, fact_sum);
                                } else if a == 0 && b == 0 && c != 0 {
                                    fact_sum = FACTORIALS[c]
                                        + FACTORIALS[d]
                                        + FACTORIALS[e]
                                        + FACTORIALS[f]
                                        + FACTORIALS[g];
                                    output = format!(
                                        "{}! + {}! + {}! + {}! + {}! = {}",
                                        c, d, e, f, g, fact_sum
                                    );
                                } else if a == 0 && b != 0 {
                                    fact_sum = FACTORIALS[b]
                                        + FACTORIALS[c]
                                        + FACTORIALS[d]
                                        + FACTORIALS[e]
                                        + FACTORIALS[f]
                                        + FACTORIALS[g];
                                    output = format!(
                                        "{}! + {}! + {}! + {}! + {}! + {}! = {}",
                                        b, c, d, e, f, g, fact_sum
                                    );
                                } else if a != 0 {
                                    fact_sum = FACTORIALS[a]
                                        + FACTORIALS[b]
                                        + FACTORIALS[c]
                                        + FACTORIALS[d]
                                        + FACTORIALS[e]
                                        + FACTORIALS[f]
                                        + FACTORIALS[g];
                                    output = format!(
                                        "{}! + {}! + {}! + {}! + {}! + {}! + {}! = {}",
                                        a, b, c, d, e, f, g, fact_sum
                                    );
                                }
                                if fact_sum == num {
                                    println!("{} = {} !!!", output, num);
                                    ret.push(num);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("sum({:?}) = {}", ret, ret.iter().sum::<u32>());
}

// Theoretical maximum:
// sum(9!) = N * 9!
// Require this to be bigger/equal 10^{N-1}
// N = 8
// Therefore the max is 1e7.
