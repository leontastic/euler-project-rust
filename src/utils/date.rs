pub mod display;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Date {
    pub year: usize,  // e.g. 1900, 1901..
    pub month: usize, // e.g. 0-11
    pub day: usize,   // e.g. 0-30
}

fn get_days_in_month(month: usize, year: usize) -> usize {
    match month {
        0 => 31,
        1 => {
            if (year % 4 == 0 && !(year % 100 == 0)) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        2 => 31,
        3 => 30,
        4 => 31,
        5 => 30,
        6 => 31,
        7 => 31,
        8 => 30,
        9 => 31,
        10 => 30,
        11 => 31,
        _ => panic!("Invalid month index {}", month),
    }
}

impl Date {
    pub fn new(year: usize, month: usize, day: usize) -> Self {
        Self { year, month, day }
    }
}

pub struct DateIterator(pub Date);

impl Iterator for DateIterator {
    type Item = Date;
    fn next(&mut self) -> Option<Date> {
        let current = self.0;

        let next_month = (current.day + 1) >= get_days_in_month(current.month, current.year);
        let next_year = next_month && current.month + 1 >= 12;

        let new_day = if next_month { 0 } else { current.day + 1 };
        let new_month = if next_month {
            if next_year {
                0
            } else {
                current.month + 1
            }
        } else {
            current.month
        };
        let new_year = if next_year {
            current.year + 1
        } else {
            current.year
        };

        let new_date = Date::new(new_year, new_month, new_day);

        self.0 = new_date;

        Some(new_date)
    }
}
