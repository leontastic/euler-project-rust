use crate::problems::parameters::Parameters;
use crate::utils::math::log_10;

pub fn solve(parameters: Parameters) -> String {
    if let Parameters::P4 { digits } = parameters {
        let upper_limit = u32::pow(10, digits);
        let lower_limit = u32::pow(10, digits - 1);

        let mut max_palindrome_global: u32 = 0;

        for n in (lower_limit..upper_limit).rev() {
            let mut max_palindrome_local: u32 = 0;
            for m in (n..upper_limit).rev() {
                if is_palindrome(n * m) {
                    max_palindrome_local = n * m;
                    break;
                }
            }
            if max_palindrome_local > max_palindrome_global {
                max_palindrome_global = max_palindrome_local
            }
        }

        if max_palindrome_global == 0 {
            panic!("No palindromes found")
        }

        format!("{}", max_palindrome_global)
    } else {
        panic!("Invalid parameters")
    }
}

fn is_palindrome(candidate: u32) -> bool {
    reverse_number(candidate) == candidate
}

fn reverse_number(n: u32) -> u32 {
    let mut forwards = n;
    let mut reversed = 0;
    for _ in 0..log_10(n) + 1 {
        reversed *= 10;
        reversed += forwards % 10;
        forwards /= 10;
    }
    reversed
}

#[test]
fn reverse_number_correctness() {
    for n in 1..100 {
        if n < 10 {
            assert_eq!(reverse_number(n), n);
        } else if n % 11 == 0 {
            assert_eq!(reverse_number(n), n);
        } else {
            assert_ne!(reverse_number(n), n);
        }
    }
    assert_eq!(reverse_number(102), 201);
    assert_eq!(reverse_number(112), 211);
    assert_eq!(reverse_number(121), 121);
    assert_eq!(reverse_number(1101), 1011);
    assert_eq!(reverse_number(12345), 54321);
    assert_eq!(reverse_number(10001), 10001);
    assert_eq!(reverse_number(11001), 10011);
    assert_eq!(reverse_number(10011), 11001);
    assert_eq!(reverse_number(90009), 90009);
    assert_eq!(reverse_number(99009), 90099);
    assert_eq!(reverse_number(90099), 99009);
    assert_eq!(reverse_number(9999), 9999);
    assert_eq!(reverse_number(99999), 99999);
}
