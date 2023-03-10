use super::Solve;

pub struct Parameters {
    pub input: &'static str,
}

static A: char = 'a';

fn alphabetical_value_char(c: char) -> usize {
    c.to_ascii_lowercase() as usize - A as usize + 1
}

fn alphabetical_value(s: &str) -> usize {
    s.chars().map(|c| alphabetical_value_char(c)).sum()
}

#[test]
fn alphabetical_value_char_correctness() {
    assert_eq!(alphabetical_value_char('a'), 1);
    assert_eq!(alphabetical_value_char('A'), 1);
    assert_eq!(alphabetical_value_char('z'), 26);
    assert_eq!(alphabetical_value_char('Z'), 26);
}

#[test]
fn alphabetical_value_correctness() {
    assert_eq!(alphabetical_value("COLIN"), 53);
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { input } = *self;

        let mut names = input
            .split(",")
            .map(|with_quotes| with_quotes.trim_matches('"'))
            .collect::<Vec<&str>>();

        names.sort_by(|&a, &b| a.cmp(&b));

        let total_name_scores = names
            .into_iter()
            .enumerate()
            .map(|(index, name)| (index + 1) * alphabetical_value(name))
            .sum::<usize>();

        Ok(Some(format!("{}", total_name_scores)))
    }
}
