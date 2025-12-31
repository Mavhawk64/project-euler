const DAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
fn main() {
    let mut counter: u32 = 0;
    // (Tues Jan 1) 1901 -> (Dec 31) 2000
    let mut day_pointer: usize = 2; // 0 - sunday, 1 - monday,..., 6 - saturday
    for y in 1901..2001 {
        for m in 1..13 as usize {
            for d in 1..get_days(m as u32, y) + 1 {
                println!("{} {} {}, {}", DAYS[day_pointer], MONTHS[m - 1], d, y);
                if d == 1 && day_pointer == 0 {
                    counter += 1;
                }
                day_pointer = (day_pointer + 1) % 7;
            }
        }
    }
    println!("{} times the first day of the month was a Sunday!", counter);
}

// How many 1st day sundays from 1901-2001[excl]?
fn get_days(m: u32, y: u32) -> u32 {
    if m == 2 {
        return 28
            + (if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
                1
            } else {
                0
            });
    }
    if m == 1 || m == 3 || m == 5 || m == 7 || m == 8 || m == 10 || m == 12 {
        return 31;
    }
    return 30;
}
