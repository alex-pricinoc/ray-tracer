use crate::{EPSILON, F};

pub trait FuzzyEq<T> {
    fn fuzzy_eq(self, other: T) -> bool;
}

impl FuzzyEq<F> for F {
    fn fuzzy_eq(self, other: Self) -> bool {
        (self - other).abs() < EPSILON
    }
}

#[macro_export]
macro_rules! assert_fuzzy_eq {
    ($left:expr, $right:expr) => {{
        match ($left, $right) {
            (left_val, right_val) => {
                if !left_val.fuzzy_eq(right_val) {
                    panic!("assertion failed: {left_val:?} is not fuzzy equal to {right_val:?}");
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_fuzzy_ne {
    ($left:expr, $right:expr) => {{
        match ($left, $right) {
            (left_val, right_val) => {
                if left_val.fuzzy_eq(right_val) {
                    panic!("assertion failed: {left_val:?} is fuzzy equal to {right_val:?}");
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_equal() {
        assert_fuzzy_eq!(1.0, 1.0);
        assert_fuzzy_eq!(0.1, 0.1);
        assert_fuzzy_eq!(0.00005, 0.00006);
        assert_fuzzy_eq!(0.000_000_9, -0.000_000_1);
        assert_fuzzy_eq!(0.000_000_923_423_4, -0.000_000_123_423_4);
    }

    #[test]
    fn should_not_be_equal() {
        assert_fuzzy_ne!(1.0, -1.0);
        assert_fuzzy_ne!(-0.1, 0.1);
        assert_fuzzy_ne!(0.0005, 0.0006);
    }
}
