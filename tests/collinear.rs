use rust_symm::{alg::get_lines_of_sym, model::{Line, Point}};
use std::collections::HashSet;

fn lines(v: &[(f64, f64)]) -> HashSet<Line> {
    get_lines_of_sym(&v.iter().map(|&(x, y)| Point::new(x, y)).collect(), None)
}
// The x-axis is 0x + by + 0 = 0.
fn has_x_axis(l: &HashSet<Line>) -> bool { l.iter().any(|l| l.a == 0.0 && l.c == 0.0) }

#[test]
fn two_points() {
    let l = lines(&[(0., 0.), (1., 0.)]);
    assert_eq!(l.len(), 2);
    assert!(has_x_axis(&l));
}
#[test]
fn symmetric_collinear() {
    let l = lines(&[(0., 0.), (1., 0.), (2., 0.)]);
    assert_eq!(l.len(), 2);
    assert!(has_x_axis(&l));
}
#[test]
fn asymmetric_collinear() {
    let l = lines(&[(0., 0.), (1., 0.), (5., 0.)]);
    assert_eq!(l.len(), 1);
    assert!(has_x_axis(&l));
}
#[test]
fn non_collinear_gets_no_through_line() {
    assert_eq!(lines(&[(0., 0.), (2., 0.), (1., 5.)]).len(), 1);
}
