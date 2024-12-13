use std::{collections::VecDeque, convert::identity};

use fxhash::{FxHashMap, FxHashSet};
use utils::{parse_with_lens, DELTAS4};

pub fn solve(s: &str) -> (usize, usize) {
    let (rows, cols, grid) = build(s);
    let mut seen = FxHashSet::default();
    let (mut p1, mut p2) = (0, 0);
    for row in 0..rows {
        for col in 0..cols {
            if seen.insert([row as i32, col as i32]) {
                let (inside, outside) = bfs(&grid, row as i32, col as i32);
                p1 += inside.len() * outside.len();
                p2 += inside.len() * count_sides(outside);
                seen.extend(inside);
            }
        }
    }
    (p1, p2)
}

fn count_sides(set: FxHashSet<[i32; 4]>) -> usize {
    let mut res = 0;
    let mut seen = FxHashSet::default();
    for &p in set.iter() {
        if !seen.insert(p) {
            continue;
        }
        let mut queue = VecDeque::from([p]);
        while let Some([row, col, dr, dc]) = queue.pop_front() {
            for d in [-1, 1] {
                if dr == 0 {
                    // dc==1 up-down fence
                    let nr = row + d;
                    if set.contains(&[nr, col, dr, dc]) && seen.insert([nr, col, dr, dc]) {
                        queue.push_back([nr, col, dr, dc]);
                    }
                } else {
                    // dr==1 left-right fence
                    let nc = col + d;
                    if set.contains(&[row, nc, dr, dc]) && seen.insert([row, nc, dr, dc]) {
                        queue.push_back([row, nc, dr, dc]);
                    }
                }
            }
        }
        res += 1;
    }
    res
}

fn build(s: &str) -> (usize, usize, FxHashMap<[i32; 2], u8>) {
    let ((rows, cols), it) = parse_with_lens(s, &identity);
    let mut grid = FxHashMap::default();
    for ((row, col), b) in it {
        grid.insert([row as i32, col as i32], b);
    }
    (rows, cols, grid)
}

fn bfs(
    grid: &FxHashMap<[i32; 2], u8>,
    row: i32,
    col: i32,
) -> (FxHashSet<[i32; 2]>, FxHashSet<[i32; 4]>) {
    let val = grid[&[row, col]];
    let mut inside = FxHashSet::default();
    let mut outside = FxHashSet::default();
    let mut queue = VecDeque::from([[row, col]]);
    inside.insert([row, col]);
    while let Some([row, col]) = queue.pop_front() {
        for [dr, dc] in DELTAS4 {
            let nr = row + dr;
            let nc = col + dc;
            if grid.get(&[nr, nc]).is_some_and(|&v| v == val) && (nr != row || nc != col) {
                if inside.insert([nr, nc]) {
                    queue.push_back([nr, nc]);
                }
            } else {
                outside.insert([nr, nc, dr, dc]);
            }
        }
    }
    (inside, outside)
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
        fn run(s: &str, n1: usize, n2: usize) {
            let (p1, p2) = solve(s);
            assert_eq!(p1, n1);
            assert_eq!(p2, n2);
        }
        run(TEST1, 140, 80);
        run(TEST2, 772, 436);
        run(TEST3, 1930, 1206);
        run(INPUT, 1424006, 858684);
    }
}
