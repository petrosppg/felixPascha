fn main() {
    let year = 2023; // Replace this with the year you want to calculate Easter for

    // Calculate Western Easter date
    let (western_month, western_day) = calculate_western_easter(year);
    println!("Western Easter date for {}: {:02}.{:02}.{}", year, western_day, western_month, year);

    // Calculate Orthodox Easter dates
    let (orthodox_julian_month, orthodox_julian_day) = calculate_orthodox_easter_julian(year);
    let (orthodox_gregorian_month, orthodox_gregorian_day) = calculate_orthodox_easter_gregorian(year);
    println!("Orthodox Easter date for {} (Julian calendar): {:02}.{:02}.{}", year, orthodox_julian_day, orthodox_julian_month, year);
    println!("Orthodox Easter date for {} (Gregorian calendar): {:02}.{:02}.{}", year, orthodox_gregorian_day, orthodox_gregorian_month, year);
}

fn calculate_western_easter(year: i32) -> (i32, i32) {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = (h + l - 7 * m + 114) % 31 + 1;
    (month, day)
}

fn calculate_orthodox_easter_julian(year: i32) -> (i32, i32) {
    let a = year % 4;
    let b = year % 7;
    let c = year % 19;
    let d = (19 * c + 15) % 30;
    let e = (2 * a + 4 * b - d + 34) % 7;
    let f = d + e + 114;
    let month = f / 31;
    let day = f % 31 + 1;
    (month, day)
}

fn calculate_orthodox_easter_gregorian(year: i32) -> (i32, i32) {
        let a = year % 19;
    let b = year % 7;
    let c = year % 4;

    let d = (19 * a + 16) % 30;
    let e = (2 * c + 4 * b + 6 * d) % 7;
    let f = (19 * a + 16) % 30;

    let key = f + e + 3;
    let (month, day) = if key > 30 {
        (5, key - 30)
    } else {
        (4, key)
    };
    (month, day)
}
