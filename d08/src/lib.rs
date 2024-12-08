use std::{
    collections::{HashMap, HashSet},
    convert::identity,
};

use itertools::Itertools;
use num_complex::Complex;
use utils::parse_with_lens;

pub fn p1(s: &str) -> usize {
    let (graph, xmax, ymax) = parse(s);
    let mut set = HashSet::new();
    for v in graph.values() {
        for (a, b) in v.iter().cartesian_product(v.iter()).filter(|(a, b)| a != b) {
            let delta = b - a;
            set.extend(
                [a - delta, b + delta]
                    .into_iter()
                    .filter(|c| check(ymax, xmax, c)),
            );
        }
    }
    set.len()
}

pub fn p2(s: &str) -> usize {
    let (graph, xmax, ymax) = parse(s);
    let mut set = HashSet::new();
    for v in graph.values() {
        for (a, b) in v.iter().cartesian_product(v.iter()).filter(|(a, b)| a != b) {
            let delta = b - a;
            set.extend(
                (0..)
                    .map(|i| a + i * delta)
                    .take_while(|c| check(ymax, xmax, c)),
            );
            set.extend(
                (1..)
                    .map(|i| a - i * delta)
                    .take_while(|c| check(ymax, xmax, c)),
            );
        }
    }
    set.len()
}

fn check(ymax: i32, xmax: i32, c: &Complex<i32>) -> bool {
    (0..xmax).contains(&c.re) && (-ymax + 1..=0).contains(&c.im)
}

fn parse(s: &str) -> (HashMap<u8, Vec<Complex<i32>>>, i32, i32) {
    let ((rows, cols), it) = parse_with_lens(s, &identity);
    let graph: HashMap<_, Vec<_>> = it.fold(HashMap::new(), |mut acc, ((row, col), b)| {
        if b.is_ascii_alphanumeric() {
            acc.entry(b)
                .or_default()
                .push(Complex::new(col as i32, -(row as i32)));
        }
        acc
    });
    let [xmax, ymax] = [cols, rows].map(|v| v as i32);
    (graph, xmax, ymax)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    const TEST: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(SAMPLE), 14);
        assert_eq!(p1(TEST), 285);

        assert_eq!(p2(SAMPLE), 34);
        assert_eq!(p2(TEST), 944);
    }
}
