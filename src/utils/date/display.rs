use super::Date;
use std::fmt::{Display, Formatter, Result};

fn get_month(index: usize) -> &'static str {
    match index {
        0 => "January",
        1 => "Feburary",
        2 => "March",
        3 => "April",
        4 => "May",
        5 => "June",
        6 => "July",
        7 => "August",
        8 => "September",
        9 => "October",
        10 => "November",
        11 => "December",
        _ => panic!("Invalid month index {}", index),
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} {} {}",
            self.year,
            get_month(self.month),
            self.day + 1
        )
    }
}

#[test]
fn display_date_correctness() {
    assert_eq!(format!("{}", Date::new(1900, 0, 0)), "1900 January 1");
    assert_eq!(format!("{}", Date::new(1999, 11, 24)), "1999 December 25");
}
