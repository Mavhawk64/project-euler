fn main() {
    let mut s = 0;
    for i in 1..1001 {
        s += remove_whitespace(&map_num_to_words(i)).len();
        println!(
            "{} = {}",
            map_num_to_words(i),
            remove_whitespace(&map_num_to_words(i)).len()
        );
    }
    println!("TOTAL: {}", s);
}

fn map_num_to_words(n: u32) -> String {
    let ones = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    if n >= 1000 {
        return "one thousand".to_string();
    }

    let hundred: usize = (n / 100) as usize;
    let ten: usize = ((n % 100) / 10) as usize;
    let one: usize = (n % 10) as usize;

    let hundred_part = if hundred > 0 {
        format!("{} hundred", ones[hundred])
    } else {
        "".to_string()
    };

    let and_part = if hundred > 0 && (ten > 0 || one > 0) {
        " and "
    } else {
        ""
    };

    let remainder_part = if ten == 1 {
        teens[one].to_string()
    } else {
        let space = if ten > 1 && one > 0 { " " } else { "" };
        format!("{}{}{}", tens[ten], space, ones[one])
    };

    return format!("{}{}{}", hundred_part, and_part, remainder_part)
        .trim()
        .to_string();
}

// Source - https://stackoverflow.com/a/57063944
// Posted by JayDepp, modified by community. See post 'Timeline' for change history
// Retrieved 2025-12-31, License - CC BY-SA 4.0

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
