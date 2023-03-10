pub fn factorize(n: usize) -> Vec<usize> {
    let mut factors_low = Vec::new();
    let mut factors_high = Vec::new();
    let mut i = 1;
    loop {
        let square = i * i;
        if square > n {
            break;
        } else if square == n {
            factors_low.push(i);
            break;
        } else if n % i == 0 {
            factors_low.push(i);
            factors_high.push(n / i);
        }
        i += 1;
    }
    factors_high.reverse();
    let mut factors = Vec::new();
    factors.append(&mut factors_low);
    factors.append(&mut factors_high);
    factors.shrink_to_fit();
    factors
}

#[test]
fn factorize_correctness() {
    let correct_factors = [
        (1, vec![1]),
        (2, vec![1, 2]),
        (3, vec![1, 3]),
        (4, vec![1, 2, 4]),
        (5, vec![1, 5]),
        (6, vec![1, 2, 3, 6]),
        (7, vec![1, 7]),
        (8, vec![1, 2, 4, 8]),
        (9, vec![1, 3, 9]),
        (10, vec![1, 2, 5, 10]),
        (11, vec![1, 11]),
        (12, vec![1, 2, 3, 4, 6, 12]),
        (13, vec![1, 13]),
        (14, vec![1, 2, 7, 14]),
        (15, vec![1, 3, 5, 15]),
        (16, vec![1, 2, 4, 8, 16]),
        (17, vec![1, 17]),
        (18, vec![1, 2, 3, 6, 9, 18]),
        (19, vec![1, 19]),
        (20, vec![1, 2, 4, 5, 10, 20]),
    ];

    for (n, factors) in correct_factors {
        assert_eq!(factorize(n), factors);
    }
}
