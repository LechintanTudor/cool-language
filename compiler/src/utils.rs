/// Hashes the given string.
pub fn hash_str(value: &str) -> u64 {
    value.chars().fold(7_u64, |hash, c| hash.wrapping_mul(31).wrapping_add(u64::from(c)))
}

/// Returns whether `n` is prime.
pub fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 {
        return false;
    } else {
        let mut i = 3;

        while i * i <= n {
            if n % i == 0 {
                return false;
            }

            i += 2;
        }

        true
    }
}

/// Returns the next prime number greater than or equal to `n`.
pub fn next_prime(n: usize) -> usize {
    for i in n..usize::MAX {
        if is_prime(i) {
            return i;
        }
    }

    panic!("Prime number does not fit u64");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(5));
        assert!(is_prime(7));

        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(!is_prime(4));
    }

    #[test]
    fn test_next_prime() {
        assert_eq!(next_prime(2), 2);
        assert_eq!(next_prime(4), 5);
        assert_eq!(next_prime(5), 5);
    }
}
