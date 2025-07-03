use std::collections::HashSet;

use lines_of_symmetry::{alg::get_lines_of_sym, model::Point};

fn main() {
    let test_cases = vec![
        HashSet::from([
            Point::new(1.0, 0.),
            Point::new(0., 1.),
            Point::new(2., 0.),
            Point::new(0., 2.),
        ]),
        HashSet::from([
            Point::new(1.0, 0.),
            Point::new(0., 1.),
            Point::new(2., 1.),
            Point::new(1., 2.),
        ]),
        HashSet::from([
            Point::new(-2., -1.),
            Point::new(-1., -0.5),
            Point::new(0., 0.),
            Point::new(3., 1.5),
        ]),
        HashSet::from([Point::new(0., 0.)]),
    ];

    for (i, case) in test_cases.iter().enumerate() {
        let lines = get_lines_of_sym(case, Some(true));
        println!("Test case {}:", i + 1);
        println!("  Points: {:?}", case);
        for line in &lines {
            println!(
                "  Line: a = {:.4}, b = {:.4}, c = {:.4}",
                line.a, line.b, line.c
            );
        }
        println!();
    }
}
