use rust_symm::{alg::get_lines_of_sym, model::Point};

// A w by h rectangle with its lower left corner at (o, o).
fn count(o: f64, w: f64, h: f64) -> usize {
    let corners = [(o, o), (o + w, o), (o, o + h), (o + w, o + h)];
    let points = corners.iter().map(|&(x, y)| Point::new(x, y)).collect();
    get_lines_of_sym(&points, None).len()
}

#[test]
fn rectangles_far_from_origin() {
    for o in [0.0, 1e6, 1e8] {
        assert_eq!(count(o, 1.0, 1.0), 4, "square at offset {o:e}");
        assert_eq!(count(o, 3.0, 2.0), 2, "rectangle at offset {o:e}");
    }
}
