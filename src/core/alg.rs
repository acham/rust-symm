use crate::model::{Line, Point, UnorderedPointPair};
use std::collections::{HashMap, HashSet};

/// Returns all lines of symmetry for a given set of points.
///
/// A line of symmetry is defined as a line such that for every point in the set,
/// there exists another point in the set that is its reflection across that line.
///
/// # Arguments
///
/// * `points` - A set of 2D points for which to find lines of symmetry.
/// * `high_degree_expected` - An optional flag indicating whether the input set
///   is expected to have a high degree of partial symmetry (many lines across
///   which many points, but not necessarily all, are symmetric). If `None`,
///   defaults to `true`.
///
/// # Returns
///
/// A `HashSet` containing all lines of symmetry for the input set of points.
///
/// # Notes
///
/// - At least two points are required to define a line of symmetry.
/// - The function uses tolerance-based floating-point comparisons to account for imprecision.
/// - If the input set contains fewer than two points, an empty set is returned and a warning is printed.
/// ```
pub fn get_lines_of_sym(points: &HashSet<Point>, high_degree_expected: Option<bool>) -> HashSet<Line> {
    // Returns a set of lines of symmetry for the given set of points.
    let high_degree_expected = high_degree_expected.unwrap_or(true);

    let mut lines_set: HashSet<Line> = HashSet::new();

    if points.len() < 2 {
        eprintln!("Warning: at least 2 points needed to find lines of symmetry.");
        return lines_set;
    }

    // A set of pairs of points that can be used to generate candidate lines of symmetry.
    let mut e_line_generators: HashSet<UnorderedPointPair> = HashSet::new();

    // Add all possible pairs of points to the set of generators.
    let points_vec: Vec<&Point> = points.iter().collect();
    for i in 0..points_vec.len() {
        for j in (i + 1)..points_vec.len() {
            let unord_ppair = UnorderedPointPair::new(points_vec[i], points_vec[j]);
            e_line_generators.insert(unord_ppair);
        }
    }

    // A reusable map to track reflections of points across the candidate lines.
    let mut point_reflections: HashMap<&Point, &Point> = HashMap::new();

    // A flag to indicate whether a line that goes through all points is possible.
    let mut through_line_possible = true;

    while let Some(e_pair) = e_line_generators.iter().next().cloned() {
        // Generate candidate line
        let e_line = get_equidistant_line(&e_pair.p1, &e_pair.p2);

        // reflection covered by this line; can be removed from input pairs.
        e_line_generators.remove(&e_pair);

        let mut valid_line = true;

        point_reflections.insert(e_pair.p1, e_pair.p2);
        point_reflections.insert(e_pair.p2, e_pair.p1);

        for point in points {
            /* Check that all input points have a reflection across the line in the input set */
            if let None = point_reflections.get(point) {
                // Input point not yet in the reflections.
                let reflection = e_line.get_reflected_point(point);

                if reflection == *point {
                    // Point is on the line, is its own reflection.
                    if !point_reflections.contains_key(point) {
                        point_reflections.insert(point, point);
                    }
                } else {
                    // Reflection is a separate point.
                    if points.contains(&reflection) {
                        // Reflection is in the input set.
                        let reflection_in_input = points.get(&reflection).unwrap();
                        point_reflections.insert(point, reflection_in_input);
                        point_reflections.insert(reflection_in_input, point);

                        /*
                         * This reflection has been covered; it can be removed from the set of generating pairs, regardless
                         * of whether the candidate line is a line of symmetry.
                         */
                        let covered_pair = UnorderedPointPair::new(point, reflection_in_input);

                        if e_line_generators.contains(&covered_pair) {
                            e_line_generators.remove(&covered_pair);
                        }
                    } else {
                        /*
                        Reflection is not in the input set, so this line is not valid.
                        If a high degree of partial symmetry is expected, don't break, because 
                        we can still use this line to remove pairs of points that are symmetric across it.
                        */
                        valid_line = false;

                        if !high_degree_expected {
                            break;
                        }
                    }
                }
            }
        }

        if valid_line {
            if through_line_possible {
                through_line_possible = false;
            }

            if !lines_set.contains(&e_line) {
                lines_set.insert(e_line);
            }
        }

        point_reflections.clear();
    }

    /* Only possible missing line of symmetry: a line that goes through all the points. */
    if through_line_possible && points_vec.len() >= 2 {
        let p1 = points_vec[0];
        let p2 = points_vec[1];
        let through_line = get_through_line(p1, p2);

        let mut through_line_valid = true;
        for point in &points_vec[2..] {
            if !through_line.is_point_on_line(*point) {
                through_line_valid = false;
                break;
            }
        }

        if through_line_valid && !lines_set.contains(&through_line) {
            lines_set.insert(through_line);
        }
    }

    lines_set
}

pub fn get_equidistant_line(p1: &Point, p2: &Point) -> Line {
    // Returns a line that is equidistant from p1 and p2
    let a = p2.x - p1.x;
    let b = p2.y - p1.y;
    let c = 0.5 * (p1.x.powf(2.0) + p1.y.powf(2.0) - p2.x.powf(2.0) - p2.y.powf(2.0));

    Line::new(a, b, c)
}

pub fn get_through_line(p1: &Point, p2: &Point) -> Line {
    // Returns a line that goes through p1 and p2
    let a = p2.y - p1.y;
    let b = p1.x - p2.x;
    let c = -(a * p1.x + b * p1.y);

    Line::new(a, b, c)
}
