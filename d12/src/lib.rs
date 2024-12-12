use std::{collections::VecDeque, convert::identity};

use fxhash::FxHashSet;
use utils::{deltas, parse_with_lens, DELTAS4};

pub fn p1(s: &str) -> usize {
    let ((rows, cols), it) = parse_with_lens(s, &identity);
    let mut grid = vec![vec![0; cols]; rows];
    for ((row, col), b) in it {
        grid[row][col] = b;
    }
    let mut seen = vec![vec![false; cols]; rows];
    let mut res = 0;
    for (row, r) in grid.iter().enumerate() {
        for (col, &b) in r.iter().enumerate() {
            if !seen[row][col] {
                seen[row][col] = true;
                res += bfs(&grid, &mut seen, row, col);
            }
        }
    }
    res
}

fn bfs(grid: &[Vec<u8>], seen: &mut [Vec<bool>], row: usize, col: usize) -> usize {
    let mut inside = FxHashSet::default();
    let mut queue = VecDeque::from([(row, col, grid[row][col])]);
    inside.insert([row, col]);
    let mut n_count = 0; // borders in the same region
    while let Some((row, col, b)) = queue.pop_front() {
        for ((nr, nc), _) in deltas(row, col) {
            if grid
                .get(nr)
                .is_some_and(|r| r.get(nc).is_some_and(|&v| v == b))
                && (nr != row || nc != col)
            {
                n_count += 1;
                if !seen[nr][nc] {
                    seen[nr][nc] = true;
                    inside.insert([nr, nc]);
                    queue.push_back((nr, nc, b));
                }
            }
        }
    }
    let n = inside.len();
    n * (4 * n - n_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    const TEST2: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

    const TEST3: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(TEST1), 140);
        assert_eq!(p1(TEST2), 772);
        assert_eq!(p1(TEST3), 1930);
        assert_eq!(p1(INPUT), 1424006);
    }
}
