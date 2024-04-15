pub fn main() {
    let hoy = Time::Day(Month::February);
    let segundos = value_in_seconds(hoy);
    println!("Segundos en un día de febrero: {}", segundos);
}

fn matchers() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

enum Time {
    Second,
    Minute,
    Hour,
    Day(Month),
}

fn value_in_seconds(time: Time) -> u32 {
    match time {
        Time::Second => 1,
        Time::Minute => 60,
        Time::Hour => 3600,
        Time::Day(month) => {
            match month {
                Month::January => 31 * 24 * 3600,
                Month::February => 28 * 24 * 3600,
                Month::March => 31 * 24 * 3600,
                Month::April => 30 * 24 * 3600,
                Month::May => 31 * 24 * 3600,
                Month::June => 30 * 24 * 3600,
                Month::July => 31 * 24 * 3600,
                Month::August => 31 * 24 * 3600,
                Month::September => 30 * 24 * 3600,
                Month::October => 31 * 24 * 3600,
                Month::November => 30 * 24 * 3600,
                Month::December => 31 * 24 * 3600,
            }
        
        }
    }
}

enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}