fn main() {
    let mut names = get_names();
    names.sort();
    // println!("{:#?}", names);
    let mut tot: u128 = 0;
    let mut cnt: u32 = 1;
    for i in names {
        tot += (get_score(i) * cnt) as u128;
        cnt += 1;
    }
    println!("Total:\n{}", tot);
}

fn get_names() -> Vec<String> {
    let input = include_str!("names.txt");

    return input
        .split(',')
        .map(|name| name.trim_matches('"').to_string())
        .collect();
}

fn get_score(name: String) -> u32 {
    let mut score: u32 = 0;
    for c in name.chars() {
        score += (c as u32) - 64; // A = 65 -> 1
    }
    return score;
}
