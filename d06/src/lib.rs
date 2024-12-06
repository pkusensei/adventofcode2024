use std::{collections::HashSet, convert::identity};

use num_complex::Complex;

pub fn solve(s: &str) -> (usize, usize) {
    let (rows, cols, grid, mut y, mut x) = parse(s);
    let start = [y, x];
    let mut dir = Complex::new(0i8, -1);
    let mut visited = HashSet::new();
    while (0..rows).contains(&y) && (0..cols).contains(&x) {
        visited.insert([y, x]);
        let Some((nx, ny)) = step(dir, x, y) else {
            break;
        };
        match grid.get(ny).and_then(|r| r.get(nx)) {
            None => break,
            Some(b'#') => dir *= Complex::I,
            Some(_) => [x, y] = [nx, ny],
        }
    }
    let p1 = visited.len();
    visited.remove(&start);

    let mut p2 = 0;
    for block in visited.into_iter() {
        p2 += usize::from(check_loop(start, &grid, rows, cols, block))
    }
    (p1, p2)
}

fn check_loop(
    start: [usize; 2],
    grid: &[Vec<u8>],
    rows: usize,
    cols: usize,
    block: [usize; 2],
) -> bool {
    let [mut y, mut x] = start;
    let mut dir = Complex::new(0i8, -1);
    let mut seen = HashSet::new();
    while (0..rows).contains(&y) && (0..cols).contains(&x) {
        if !seen.insert((x, y, dir)) {
            return true;
        }
        let Some((nx, ny)) = step(dir, x, y) else {
            break;
        };
        if [ny, nx] == block {
            dir *= Complex::I;
            continue;
        }
        match grid.get(ny).and_then(|r| r.get(nx)) {
            None => break,
            Some(b'#') => dir *= Complex::I,
            Some(_) => [x, y] = [nx, ny],
        }
    }
    false
}

fn step(dir: Complex<i8>, x: usize, y: usize) -> Option<(usize, usize)> {
    match (dir.re, dir.im) {
        (1, 0) => x.checked_add(1).zip(Some(y)),
        (-1, 0) => x.checked_sub(1).zip(Some(y)),
        (0, 1) => Some(x).zip(y.checked_add(1)),
        (0, -1) => Some(x).zip(y.checked_sub(1)),
        _ => None,
    }
}

fn parse(s: &str) -> (usize, usize, Vec<Vec<u8>>, usize, usize) {
    let ((rows, cols), it) = utils::parse_with_lens(s, &identity);
    let mut grid = vec![vec![0; cols]; rows];
    let [mut y, mut x] = [0, 0];
    for ((r, c), v) in it {
        grid[r][c] = v;
        if v == b'^' {
            [y, x] = [r, c];
        }
    }
    (rows, cols, grid, y, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    const TEST: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        let (p1, p2) = solve(SAMPLE);
        assert_eq!(p1, 41);
        assert_eq!(p2, 6);

        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 5534);
        assert_eq!(p2, 2262);
    }
}
