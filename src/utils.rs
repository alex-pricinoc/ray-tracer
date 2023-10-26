use crate::EPSILON;

pub trait FuzzyEq<T> {
    fn fuzzy_eq(&self, other: T) -> bool;
}

impl FuzzyEq<f64> for f64 {
    fn fuzzy_eq(&self, other: Self) -> bool {
        (*self - other).abs() < EPSILON
    }
}

#[macro_export]
macro_rules! assert_fuzzy_eq {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !left_val.fuzzy_eq(*right_val) {
                    panic!("assertion failed: {left_val:?} is not fuzzy equal to {right_val:?}");
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_fuzzy_ne {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val.fuzzy_eq(*right_val) {
                    panic!("assertion failed: {left_val:?} is fuzzy equal to {right_val:?}");
                }
            }
        }
    }};
}
