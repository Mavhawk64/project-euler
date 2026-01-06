use std::collections::HashSet;
use std::fmt;
fn main() {
    let pandigital_nums: HashSet<u32> = preprocess_pandigital_nines();
    // Generate NumChunks up to 9999 because we require that n > 1, and a NumChunk is such that
    // mults[1] | mults[2] | ...
    // The biggest number we can have with the minimum n would be 9999 because it is a 4-digit number
    // that generates a 5-digit mults[2] value.
    // Thus, 9999 x 1 = 9999 | 9999 x 2 = 19,998 --> 999_919_998
    let mut numchunks: Vec<NumChunk> = Vec::new();
    for i in 0..10_000 {
        numchunks.push(NumChunk::new(i));
    }
    for i in numchunks.into_iter().rev() {
        if pandigital_nums.contains(&i.get_chunk()) {
            println!("NumChunk:\n{}\nNumber:\n{}", i, i.get_chunk());
            break;
        }
    }
}

#[derive(Debug, Clone)]
struct NumChunk {
    num: u32,
    mults: Vec<u32>,
}

impl NumChunk {
    fn new(num: u32) -> Self {
        let mut mults: Vec<u32> = Vec::new();
        for i in 0..10 {
            if mults.len() < 2 || chunks_leq_9_len(num * i, mults.clone()) {
                mults.push(num * i);
            }
        }
        NumChunk {
            num: num,
            mults: mults,
        }
    }

    fn get_chunk(&self) -> u32 {
        self.mults[1..]
            .iter()
            .map(|&x| x.to_string())
            .collect::<String>()
            .parse()
            .unwrap_or(0)
    }
}

impl fmt::Display for NumChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NumChunk struct[num = {}, mults={:?}]",
            self.num, self.mults
        )
    }
}

fn chunks_leq_9_len(a: u32, cat: Vec<u32>) -> bool {
    a.to_string().len() + cat[1..].iter().map(|&x| x.to_string().len()).sum::<usize>() <= 9
}

fn preprocess_pandigital_nines() -> HashSet<u32> {
    let mut pandigital_nums: HashSet<u32> = HashSet::with_capacity(362_880); // 9!
    for a in 1..10 {
        for b in 1..10 {
            if b == a {
                continue;
            }
            for c in 1..10 {
                if c == b || c == a {
                    continue;
                }
                for d in 1..10 {
                    if d == c || d == b || d == a {
                        continue;
                    }
                    for e in 1..10 {
                        if e == d || e == c || e == b || e == a {
                            continue;
                        }
                        for f in 1..10 {
                            if f == e || f == d || f == c || f == b || f == a {
                                continue;
                            }
                            for g in 1..10 {
                                if g == f || g == e || g == d || g == c || g == b || g == a {
                                    continue;
                                }
                                for h in 1..10 {
                                    if h == g
                                        || h == f
                                        || h == e
                                        || h == d
                                        || h == c
                                        || h == b
                                        || h == a
                                    {
                                        continue;
                                    }
                                    for i in 1..10 {
                                        if i == h
                                            || i == g
                                            || i == f
                                            || i == e
                                            || i == d
                                            || i == c
                                            || i == b
                                            || i == a
                                        {
                                            continue;
                                        }
                                        pandigital_nums.insert(
                                            i + 10 * h
                                                + 100 * g
                                                + 1_000 * f
                                                + 10_000 * e
                                                + 100_000 * d
                                                + 1_000_000 * c
                                                + 10_000_000 * b
                                                + 100_000_000 * a,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pandigital_nums
}
