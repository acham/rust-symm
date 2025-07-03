use crate::config;
use crate::util;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// A point in 2D space with floating-point coordinates.
///
/// Points are compared using a tolerance-based comparison to handle floating-point imprecision.
/// Coordinates must be finite and non-NaN.
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Creates a new Point with the given coordinates.
    ///
    /// # Panics
    ///
    /// Panics if either coordinate is NaN or infinite.
    pub fn new(x: f64, y: f64) -> Self {
        if !x.is_finite() || !y.is_finite() {
            panic!("Point coordinates must be finite and non-NaN");
        }
        Self { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        util::floats_equal_toler(self.x, other.x) && util::floats_equal_toler(self.y, other.y)
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let x_cmp = util::float_partial_cmp_tolerance(&self.x, &other.x);
        match x_cmp {
            Some(Ordering::Equal) => util::float_partial_cmp_tolerance(&self.y, &other.y),
            other => other,
        }
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

/// A line in 2D space represented by the equation ax + by + c = 0.
#[derive(Debug)]
pub struct Line {
    // ax + by + c = 0
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Line {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    /// Returns a hash value for the line, using the custom hash implementation.
    pub fn get_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    /// Returns the reflection of the given point `p` across this line.
    pub fn get_reflected_point(&self, p: &Point) -> Point {
        let denom = self.a.powf(2.0) + self.b.powf(2.0);
        if denom == 0.0 {
            panic!("Invalid line: a^2 + b^2 cannot be zero");
        }

        let factor = 2.0 * (self.a * p.x + self.b * p.y + self.c) / denom;
        let x_reflected = p.x - factor * self.a;
        let y_reflected = p.y - factor * self.b;

        Point::new(x_reflected, y_reflected)
    }

    /// Checks if the given point lies on this line, within floating-point tolerance.
    pub fn is_point_on_line(&self, p: &Point) -> bool {
        util::floats_equal_toler(self.a * p.x + self.b * p.y + self.c, 0.0)
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        util::float_partial_cmp_tolerance(&self.a, &other.a) == Some(Ordering::Equal)
            && util::float_partial_cmp_tolerance(&self.b, &other.b) == Some(Ordering::Equal)
            && util::float_partial_cmp_tolerance(&self.c, &other.c) == Some(Ordering::Equal)
    }
}

impl Eq for Line {}

impl Hash for Line {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let round = |x: f64| (x / config::EPSILON).round() * config::EPSILON;
        round(self.a).to_bits().hash(state);
        round(self.b).to_bits().hash(state);
        round(self.c).to_bits().hash(state);
    }
}

/// An unordered pair of points, used for symmetry calculations.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UnorderedPointPair<'a> {
    pub p1: &'a Point,
    pub p2: &'a Point,
}

impl<'a> UnorderedPointPair<'a> {
    /// Constructs a new unordered pair, ordering the points canonically.
    pub fn new(p1: &'a Point, p2: &'a Point) -> Self {
        if p1 <= p2 {
            Self { p1, p2 }
        } else {
            Self { p1: p2, p2: p1 }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Tests that lines which are equal within the configured floating-point tolerance
    /// are treated as equal by both hashing and equality, and that lines differing by
    /// more than the tolerance are treated as distinct.
    ///
    /// This ensures that the custom `Hash` and `PartialEq` implementations for `Line`
    /// are consistent and robust to floating-point imprecision.
    #[test]
    fn test_line_hash() {
        // Create lines that should be considered equal (within EPSILON)
        let l1 = Line::new(1.0, 2.0, 3.0);
        let l2 = Line::new(1.0 + config::EPSILON / 10.0, 2.0, 3.0);
        let l3 = Line::new(1.0, 2.0 + config::EPSILON / 10.0, 3.0);
        let l4 = Line::new(1.0, 2.0, 3.0 + config::EPSILON / 10.0);

        // Create lines that should be different (beyond EPSILON)
        let l5 = Line::new(1.0 + config::EPSILON, 2.0, 3.0);
        let l6 = Line::new(1.0, 2.0 + config::EPSILON, 3.0);
        let l7 = Line::new(1.0, 2.0, 3.0 + config::EPSILON);

        // Create a HashSet to test hashing
        let mut set = HashSet::new();

        // Add all lines
        set.insert(l1);
        set.insert(l2);
        set.insert(l3);
        set.insert(l4);
        set.insert(l5);
        set.insert(l6);
        set.insert(l7);

        // Lines within EPSILON should be considered the same
        assert_eq!(set.len(), 4); // l1-l4 should be one entry, l5-l7 should be separate
    }

    /// Tests the custom `PartialOrd` implementation for `Point`.
    /// Ensures that point ordering is robust to floating-point imprecision and behaves as intended.
    #[test]
    fn test_point_ordering() {
        // Test equal points (within epsilon)
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(1.0 + config::EPSILON / 2.0, 2.0 + config::EPSILON / 2.0);
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Equal));
        assert!(p1 <= p2);
        assert!(p1 >= p2);

        // Test different x-coordinates
        let p3 = Point::new(1.0, 2.0);
        let p4 = Point::new(2.0, 2.0);
        assert_eq!(p3.partial_cmp(&p4), Some(Ordering::Less));
        assert!(p3 < p4);
        assert!(p3 <= p4);
        assert!(p4 > p3);
        assert!(p4 >= p3);

        // Test equal x but different y
        let p5 = Point::new(1.0, 2.0);
        let p6 = Point::new(1.0, 3.0);
        assert_eq!(p5.partial_cmp(&p6), Some(Ordering::Less));
        assert!(p5 < p6);
        assert!(p5 <= p6);
        assert!(p6 > p5);
        assert!(p6 >= p5);

        // Test points with x difference just above epsilon
        let p7 = Point::new(1.0, 2.0);
        let p8 = Point::new(1.0 + config::EPSILON * 2.0, 2.0);
        assert_eq!(p7.partial_cmp(&p8), Some(Ordering::Less));
        assert!(p7 < p8);

        // Test points with x difference just below epsilon
        let p9 = Point::new(1.0, 2.0);
        let p10 = Point::new(1.0 + config::EPSILON / 2.0, 2.0);
        assert_eq!(p9.partial_cmp(&p10), Some(Ordering::Equal));
        assert!(p9 <= p10);
        assert!(p9 >= p10);
    }
}
