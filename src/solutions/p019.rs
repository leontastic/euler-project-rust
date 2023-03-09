use super::Solve;
use crate::utils::date::{Date, DateIterator};

pub struct Parameters {}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let count = DateIterator(Date::new(1900, 11, 30))
            .take_while(|date| date.year < 2001)
            .step_by(7)
            .filter(|date| date.day == 0)
            // .inspect(|date| println!("{} ", date))
            .count();

        Ok(Some(format!("{}", count)))
    }
}
