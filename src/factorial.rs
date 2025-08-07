pub fn compute(n: u128) -> u128 {
    (1..n+1).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_absolute_values() {
        assert_eq!(compute(0), 1);
        assert_eq!(compute(1), 1);
        assert_eq!(compute(2), 2);
        assert_eq!(compute(3), 6);
        assert_eq!(compute(4), 24);
        assert_eq!(compute(5), 120);
        assert_eq!(compute(6), 720);
        assert_eq!(compute(7), 5040);
        assert_eq!(compute(8), 40320);
        assert_eq!(compute(9), 362880);
    }

    #[test]
    fn factorial_value_relationships() {
        assert_eq!(compute(4) * 5, compute(5));
        assert_eq!(compute(16) * 17, compute(17));
        assert_eq!(compute(33) * 34, compute(34));
    }
}
