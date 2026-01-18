fn main() {
    let mut tris: Vec<u32> = Vec::new();
    let mut sqrs: Vec<u32> = Vec::new();
    let mut pnts: Vec<u32> = Vec::new();
    let mut hexs: Vec<u32> = Vec::new();
    let mut hpts: Vec<u32> = Vec::new();
    let mut octs: Vec<u32> = Vec::new();

    // this was silly... we should have just used the pattern they gave us since it's the
    // only set of 3 numbers that do that... so obv it had to have happened for the other sets
    // (otherwise it wouldn't be a question!)

    for n in 1..141 {
        let t: u32 = triangle(n);
        if (1000..10_000).contains(&t) {
            tris.push(t);
        }
        let s: u32 = square(n);
        if (1000..10_000).contains(&s) {
            sqrs.push(s);
        }
        let p: u32 = pentagonal(n);
        if (1000..10_000).contains(&p) {
            pnts.push(p);
        }
        let x: u32 = hexagonal(n);
        if (1000..10_000).contains(&x) {
            hexs.push(x);
        }
        let h: u32 = heptagonal(n);
        if (1000..10_000).contains(&h) {
            hpts.push(h);
        }
        let o: u32 = octagonal(n);
        if (1000..10_000).contains(&o) {
            octs.push(o);
        }
    }
    // println!("{} {} {} {} {} {}", tris.len(), sqrs.len(), pnts.len(), hexs.len(), hpts.len(), octs.len());
    let sets = vec![tris, sqrs, pnts, hexs, hpts];

    for &o in &octs {
        if let Some(final_path) = solve(o, o / 100, sets.clone(), vec![o]) {
            let sum: u32 = final_path.iter().sum();
            println!("Found cycle: {:?}", final_path);
            println!("Sum: {}", sum);
            break;
        }
    }
}

fn solve(
    current: u32,
    target_start: u32,
    remaining_sets: Vec<Vec<u32>>,
    path: Vec<u32>,
) -> Option<Vec<u32>> {
    // Base Case: All 6 types are used
    if remaining_sets.is_empty() {
        if current % 100 == target_start {
            return Some(path);
        }
        return None;
    }

    // Try each remaining set
    for i in 0..remaining_sets.len() {
        let set = &remaining_sets[i];
        let matches = find_beginning_matching_ending(set.clone(), current % 100);

        for &m in &matches {
            let mut next_sets = remaining_sets.clone();
            next_sets.remove(i); // Mark this polygonal type as used

            let mut next_path = path.clone();
            next_path.push(m);

            if let Some(result) = solve(m, target_start, next_sets, next_path) {
                return Some(result);
            }
        }
    }
    None
}

fn find_beginning_matching_ending(mut v: Vec<u32>, ending: u32) -> Vec<u32> {
    v.retain(|&v| v / 100 == ending);
    v
}

fn triangle(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

fn square(n: u32) -> u32 {
    n * n
}

fn pentagonal(n: u32) -> u32 {
    (n * (3 * n - 1)) / 2
}

fn hexagonal(n: u32) -> u32 {
    n * (2 * n - 1)
}

fn heptagonal(n: u32) -> u32 {
    (n * (5 * n - 3)) / 2
}

fn octagonal(n: u32) -> u32 {
    n * (3 * n - 2)
}
