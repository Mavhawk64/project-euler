use std::fs;

fn main() {
    let data: Vec<u8> = fs::read_to_string(
        "/home/maverick/repos/project-euler/solutions_51-100/problem-59/xor-decryption/src/input.txt"
    )
    .expect("Should be able to read input file")
    .trim()
    .split(',')
    .map(|s| s.parse::<u8>().expect("Invalid number"))
    .collect();

    for a in b'a'..=b'z' {
        for b in b'a'..=b'z' {
            for c in b'a'..=b'z' {
                let decrypted: Vec<u8> = data
                    .iter()
                    .zip([a, b, c].iter().cycle())
                    .map(|(&byte, &k)| byte ^ k)
                    .collect();

                let text: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&decrypted);

                // honestly, if we just check the two most common, simple (but also ~difficult for random chars) words,
                // i bet we could brute force this!
                // sure enough, "the" and "and" work, but we need to add some spaces
                // to make sure that we capture them as whole words!
                // https://en.wikipedia.org/wiki/Most_common_words_in_English
                if text.contains(" the ") && text.contains(" and ") {
                    println!("Key: {}{}{}", a as char, b as char, c as char);
                    println!("{}", text);
                    let sum: u32 = decrypted.iter().map(|&b| b as u32).sum();
                    println!("ASCII sum: {}", sum);
                    return;
                }
            }
        }
    }
}
