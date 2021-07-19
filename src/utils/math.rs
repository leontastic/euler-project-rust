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
