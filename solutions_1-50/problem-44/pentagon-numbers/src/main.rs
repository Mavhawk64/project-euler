fn main() {
    // this logic is much better.
    // Since we are tasked with minimizing the difference, D (or p_d in the code),
    // We can do some algebraic manipulation behind the scenes to figure out what we require:
    // Start with the given parameters (looping parameters in the old/ugly donotuse() method):
    // P_j, P_k.
    // Let P_s = P_k + P_j and D = P_d = P_k - P_j
    // P_k = P_d + P_j --> P_s = P_d + 2 * P_j
    // Therefore, we have our two conditions
    // p_k = p(j) + p_d has to be pentagonal
    // AND p_s = p_d + 2*p_j (or p_j + p_k) has to be pentagonal

    let mut d: u128 = 2;
    'outer: loop {
        let p_d = p(d);
        for j in 1..d {
            let p_k = p(j) + p_d;
            let p_s = p(j) + p_k;
            if check_pentagonal_num(p_k) && check_pentagonal_num(p_s) {
                println!("D = {}", p_d);
                break 'outer;
            }
        }
        d += 1;
    }
}

// This works, but at what cost...
// It's actually not necessarily a mathematically sound solution, either...
// It just happened to work.
// Btw it outputs
// fn donotrun() {
//     // PLEASE DO NOT RUN THIS
//     'outer: for j in 1..u16::MAX {
//         for k in j + 1..u16::MAX {
//             let s: u128 = p(j as u128) + p(k as u128);
//             let d: u128 = p(k as u128) - p(j as u128);
//             println!(
//                 "Pair: P_{} - P_{} = {} - {} = {}",
//                 k,
//                 j,
//                 p(k as u128),
//                 p(j as u128),
//                 d
//             );
//             if check_pentagonal_num(s) && check_pentagonal_num(d) {
//                 println!(
//                     "Pair: P_{} - P_{} = {} - {} = {}",
//                     k,
//                     j,
//                     p(k as u128),
//                     p(j as u128),
//                     d
//                 );
//                 break 'outer;
//             }
//         }
//     }
// }

fn p(n: u128) -> u128 {
    n * (3 * n - 1) / 2
}

fn n(p: u128) -> u128 {
    ((24 * p + 1).isqrt() + 1) / 6
}

fn check_pentagonal_num(pn: u128) -> bool {
    p(n(pn)) == pn
}
