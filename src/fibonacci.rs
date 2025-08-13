pub fn compute(num: u128) -> u128 {
    if num < 2 {
        return num;
    }
    let mut prev_prev: u128 = 0;
    let mut prev: u128 = 1;
    let mut cur = prev_prev + prev;
    for _ in 3..num + 1 {
        prev_prev = prev;
        prev = cur;
        cur = prev_prev + prev;
    }
    cur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_absolute_values() {
        assert_eq!(compute(0), 0);
        assert_eq!(compute(1), 1);
        assert_eq!(compute(2), 1);
        assert_eq!(compute(3), 2);
        assert_eq!(compute(4), 3);
        assert_eq!(compute(5), 5);
        assert_eq!(compute(6), 8);
        assert_eq!(compute(7), 13);
        assert_eq!(compute(8), 21);
        assert_eq!(compute(9), 34);
    }

    #[test]
    fn fibonacci_value_relationships() {
        assert_eq!(compute(3) + compute(4), compute(5));
        assert_eq!(compute(126) + compute(127), compute(128));
    }
}
