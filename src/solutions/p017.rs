use super::Solve;

pub struct Parameters {
    pub n: usize,
}

fn number_to_word(n: usize) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => "",
    }
}

fn number_to_words(n: usize) -> String {
    let mut words: String = "".to_owned();
    let mut current = n.clone();

    while current > 0 {
        if current >= 1000 {
            words += &format!("{} thousand", number_to_word(current / 1000)).to_owned();
            current %= 1000;
            if current > 0 {
                words += " ";
            }
        } else if current >= 100 {
            words += &format!("{} hundred", number_to_word(current / 100)).to_owned();
            current %= 100;
            if current > 0 {
                words += " and ";
            }
        } else if current >= 20 {
            let mut word = number_to_word(current / 10 * 10).to_string();
            current %= 10;
            if current > 0 {
                word += &format!("-{}", number_to_word(current));
                current -= current
            }
            words += &word
        } else {
            words += number_to_word(current);
            current -= current
        }
    }

    words
}

#[test]
fn number_to_words_correctness() {
    assert_eq!(number_to_words(1), "one");
    assert_eq!(number_to_words(2), "two");
    assert_eq!(number_to_words(3), "three");
    assert_eq!(number_to_words(4), "four");
    assert_eq!(number_to_words(5), "five");
    assert_eq!(number_to_words(6), "six");
    assert_eq!(number_to_words(7), "seven");
    assert_eq!(number_to_words(8), "eight");
    assert_eq!(number_to_words(9), "nine");
    assert_eq!(number_to_words(10), "ten");
    assert_eq!(number_to_words(11), "eleven");
    assert_eq!(number_to_words(12), "twelve");
    assert_eq!(number_to_words(13), "thirteen");
    assert_eq!(number_to_words(14), "fourteen");
    assert_eq!(number_to_words(15), "fifteen");
    assert_eq!(number_to_words(16), "sixteen");
    assert_eq!(number_to_words(17), "seventeen");
    assert_eq!(number_to_words(18), "eighteen");
    assert_eq!(number_to_words(19), "nineteen");
    assert_eq!(number_to_words(20), "twenty");
    assert_eq!(number_to_words(21), "twenty-one");
    assert_eq!(number_to_words(99), "ninety-nine");
    assert_eq!(number_to_words(100), "one hundred");
    assert_eq!(number_to_words(102), "one hundred and two");
    assert_eq!(number_to_words(119), "one hundred and nineteen");
    assert_eq!(number_to_words(333), "three hundred and thirty-three");
    assert_eq!(number_to_words(999), "nine hundred and ninety-nine");
    assert_eq!(number_to_words(1000), "one thousand");
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let sum_letters = (1..(n + 1))
            .map(|i| number_to_words(i).replace(" ", "").replace("-", "").len())
            .sum::<usize>();

        Ok(Some(format!("{}", sum_letters)))
    }
}
