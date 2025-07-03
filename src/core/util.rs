use std::cmp::Ordering;
use crate::config;


/// Returns the type name of a given value as a static string slice.
pub fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn float_partial_cmp_tolerance(a: &f64, b: &f64) -> Option<Ordering> {
    if a.is_finite() && b.is_finite() {
        let diff = (a - b).abs();

        if diff < config::EPSILON {
            return Some(Ordering::Equal);
        } else if a < b {
            return Some(Ordering::Less);
        } else {
            return Some(Ordering::Greater);
        }
    }

    None
}

pub fn floats_equal_toler(a: f64, b: f64) -> bool {
    (a - b).abs() < config::EPSILON
}

pub fn floats_lt_toler(a: f64, b: f64) -> bool {
    b - a > config::EPSILON
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_of() {
        let x: f64 = 0.0;
        assert_eq!(type_of(&x), "f64");
        
        let y: i32 = 0;
        assert_eq!(type_of(&y), "i32");
    }
}
