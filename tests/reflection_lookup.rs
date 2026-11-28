use rust_symm::{alg::get_lines_of_sym, model::Point};
use std::f64::consts::PI;

fn count(v: &[(f64, f64)], high_degree_expected: bool) -> usize {
    let points = v.iter().map(|&(x, y)| Point::new(x, y)).collect();
    get_lines_of_sym(&points, Some(high_degree_expected)).len()
}

// Reflections of these points are only equal to input points up to round-off.
#[test]
fn regular_polygons() {
    for k in [3, 4, 5, 6, 8, 12, 101] {
        let v: Vec<_> = (0..k)
            .map(|i| (2.0 * PI * i as f64 / k as f64).sin_cos())
            .collect();
        for high in [true, false] {
            assert_eq!(count(&v, high), k, "{k}-gon, high_degree_expected={high}");
        }
    }
}
