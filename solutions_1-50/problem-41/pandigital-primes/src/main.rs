use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
};

use primes::is_prime;
fn main() {
    let pandigital_nums: HashSet<u32> = get_pandigital_nums(); // Rather than searching 987654321 numbers, search 409113 (\sum_{n=1}^{9}n!)
    let mut sorted_nums: Vec<u32> = pandigital_nums.into_iter().collect();
    sorted_nums.sort_by(|a, b| b.cmp(a)); // Sort in descending order

    // output these to a file for later...
    let file = File::create("/home/maverick/repos/project-euler/solutions_1-50/problem-41/pandigital-primes/src/pandigital_numbers.txt").unwrap();
    let mut writer = BufWriter::new(file);

    for n in sorted_nums.iter().rev() {
        // Writes each number on a new line
        writeln!(writer, "{}", n).unwrap();
    }

    for i in sorted_nums {
        if is_prime(i as u64) {
            println!("{} is the biggest Pandigital Prime!", i);
            break;
        }
    }
}

fn get_pandigital_nums() -> HashSet<u32> {
    let mut pandigital_nums = get_pandigital_nines();
    pandigital_nums.extend(get_pandigital_eights());
    pandigital_nums.extend(get_pandigital_sevens());
    pandigital_nums.extend(get_pandigital_sixes());
    pandigital_nums.extend(get_pandigital_fives());
    pandigital_nums.extend(get_pandigital_fours());
    pandigital_nums.extend(get_pandigital_threes());
    pandigital_nums.extend([12, 21]);
    pandigital_nums.extend([1]);
    pandigital_nums
}

fn get_pandigital_nines() -> HashSet<u32> {
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

fn get_pandigital_eights() -> HashSet<u32> {
    let mut pandigital_nums: HashSet<u32> = HashSet::with_capacity(40_320); // 8!
    for b in 1..9 {
        for c in 1..9 {
            if c == b {
                continue;
            }
            for d in 1..9 {
                if d == c || d == b {
                    continue;
                }
                for e in 1..9 {
                    if e == d || e == c || e == b {
                        continue;
                    }
                    for f in 1..9 {
                        if f == e || f == d || f == c || f == b {
                            continue;
                        }
                        for g in 1..9 {
                            if g == f || g == e || g == d || g == c || g == b {
                                continue;
                            }
                            for h in 1..9 {
                                if h == g || h == f || h == e || h == d || h == c || h == b {
                                    continue;
                                }
                                for i in 1..9 {
                                    if i == h
                                        || i == g
                                        || i == f
                                        || i == e
                                        || i == d
                                        || i == c
                                        || i == b
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
                                            + 10_000_000 * b,
                                    );
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

fn get_pandigital_sevens() -> HashSet<u32> {
    let mut pandigital_nums: HashSet<u32> = HashSet::with_capacity(5_040); // 7!

    for c in 1..8 {
        for d in 1..8 {
            if d == c {
                continue;
            }
            for e in 1..8 {
                if e == d || e == c {
                    continue;
                }
                for f in 1..8 {
                    if f == e || f == d || f == c {
                        continue;
                    }
                    for g in 1..8 {
                        if g == f || g == e || g == d || g == c {
                            continue;
                        }
                        for h in 1..8 {
                            if h == g || h == f || h == e || h == d || h == c {
                                continue;
                            }
                            for i in 1..8 {
                                if i == h || i == g || i == f || i == e || i == d || i == c {
                                    continue;
                                }
                                pandigital_nums.insert(
                                    i + 10 * h
                                        + 100 * g
                                        + 1_000 * f
                                        + 10_000 * e
                                        + 100_000 * d
                                        + 1_000_000 * c,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    pandigital_nums
}

fn get_pandigital_sixes() -> HashSet<u32> {
    let mut pandigital_nums: HashSet<u32> = HashSet::with_capacity(720); // 6!

    for d in 1..7 {
        for e in 1..7 {
            if e == d {
                continue;
            }
            for f in 1..7 {
                if f == e || f == d {
                    continue;
                }
                for g in 1..7 {
                    if g == f || g == e || g == d {
                        continue;
                    }
                    for h in 1..7 {
                        if h == g || h == f || h == e || h == d {
                            continue;
                        }
                        for i in 1..7 {
                            if i == h || i == g || i == f || i == e || i == d {
                                continue;
                            }
                            pandigital_nums.insert(
                                i + 10 * h + 100 * g + 1_000 * f + 10_000 * e + 100_000 * d,
                            );
                        }
                    }
                }
            }
        }
    }
    pandigital_nums
}

fn get_pandigital_fives() -> HashSet<u32> {
    let mut pandigital_nums: HashSet<u32> = HashSet::with_capacity(120); // 5!

    for e in 1..6 {
        for f in 1..6 {
            if f == e {
                continue;
            }
            for g in 1..6 {
                if g == f || g == e {
                    continue;
                }
                for h in 1..6 {
                    if h == g || h == f || h == e {
                        continue;
                    }
                    for i in 1..6 {
                        if i == h || i == g || i == f || i == e {
                            continue;
                        }
                        pandigital_nums.insert(i + 10 * h + 100 * g + 1_000 * f + 10_000 * e);
                    }
                }
            }
        }
    }
    pandigital_nums
}

fn get_pandigital_fours() -> HashSet<u32> {
    let mut pandigital_nums: HashSet<u32> = HashSet::with_capacity(24); // 4!

    for f in 1..5 {
        for g in 1..5 {
            if g == f {
                continue;
            }
            for h in 1..5 {
                if h == g || h == f {
                    continue;
                }
                for i in 1..5 {
                    if i == h || i == g || i == f {
                        continue;
                    }
                    pandigital_nums.insert(i + 10 * h + 100 * g + 1_000 * f);
                }
            }
        }
    }
    pandigital_nums
}

fn get_pandigital_threes() -> HashSet<u32> {
    let mut pandigital_nums: HashSet<u32> = HashSet::with_capacity(6); // 3!

    for g in 1..4 {
        for h in 1..4 {
            if h == g {
                continue;
            }
            for i in 1..4 {
                if i == h || i == g {
                    continue;
                }
                pandigital_nums.insert(i + 10 * h + 100 * g);
            }
        }
    }
    pandigital_nums
}
