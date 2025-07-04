# Cartesian Symmetry

A Rust library for exploring symmetry in 2D space 
with tolerance intervals for floating-point comparisons.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-symm = "0.1"
```

See `main.rs` for example usage.

## Discussion

### Finding Lines of Symmetry (`alg::get_lines_of_sym`)

Given a set of points `S_p`, a line of symmetry for the set is defined
as a line for which, given any point `p_1` in the set, there exists a
point `p_2` in `S_p` that is the reflection of `p_1` across the
line. If `p_1` is on the line, it is its own reflection.

The problem is to find the set of all lines of symmetry `S_ls` given a
set of points `S_p`. Intuitively, the set of all lines that are
equidistant from pairs of different points in `S_p` (perpendicular
bisectors of point pairs), with the possible addition of a line that
goes through all the points (if there is such a line), forms a set
`S_lc` (set of candidate lines) that is a superset of `S_ls`. The
basic proof for this is that if there is a line `l_i` for which a
point in `S_p` is both not on the line, and does not have a reflection
across it that is also in `S_p`, then `l_i` is simply not a line of
symmetry.

As a first step, the algorithm creates a set of all distinct, order-
insensitive pairs of different points in `S_p`, called
`e_line_generators`. Then, a random pair is repeatedly picked from
this set, the perpendicular bisector line is calculated for it, and
all points in the set are checked for symmetry across this candidate
line.

Any pair of points found to constitute a reflection across a candidate
line is removed from `e_line_generators`, as the pair would generate
the same line being examined.  In addition, when a point is found
without a reflection across a particular candidate line, that is when
the candidate line is not a line of symmetry, the algorithm possibly
continues checking points in the input set for reflections across this
line, so that problematic point pairs can potentially be removed from
the generator set. This is an optimization for input sets in which a
high degree of partial symmetry is expected (parameter
`high_degree_expected: Option<bool>`).

Finally, the existence of a line going through all the points is checked.
