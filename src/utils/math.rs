pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn log_10(n: u32) -> u32 {
    if n < 10 {
        0
    } else {
        log_10(n / 10) + 1
    }
}

#[test]
fn lcm_correctness() {
    assert_eq!(lcm(3, 4), 12);
    assert_eq!(lcm(3, 5), 15);
    assert_eq!(lcm(8, 9), 72);
    assert_eq!(lcm(12, 15), 60);
}

#[test]
fn gcd_correctness() {
    assert_eq!(gcd(12, 15), 3);
    assert_eq!(gcd(25, 35), 5);
    assert_eq!(gcd(18, 24), 6);
    assert_eq!(gcd(72, 81), 9);
}

#[test]
fn log_10_correctness() {
    // test unsigned integers up to 100 million
    for exp in 0..8 {
        for n in 10u32.pow(exp)..10u32.pow(exp + 1) {
            assert_eq!(log_10(n), exp);
        }
    }
}
