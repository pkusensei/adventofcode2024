use std::{collections::VecDeque, convert::identity};

use fxhash::{FxHashMap, FxHashSet};
use utils::{parse_with_lens, DELTAS4};

pub fn p1(s: &str) -> usize {
    let (rows, cols, grid) = build(s);
    let mut seen = FxHashSet::default();
    let mut res = 0;
    for row in 0..rows {
        for col in 0..cols {
            if seen.insert([row as i32, col as i32]) {
                let (inside, outside) = bfs(&grid, row as i32, col as i32);
                res += inside.len() * outside.len();
                seen.extend(inside);
            }
        }
    }
    res
}

pub fn p2(s: &str) -> usize {
    let (rows, cols, grid) = build(s);
    let mut seen = FxHashSet::default();
    let mut res = 0;
    for row in 0..rows {
        for col in 0..cols {
            if seen.insert([row as i32, col as i32]) {
                let (inside, outside) = bfs(&grid, row as i32, col as i32);
                res += inside.len() * count_sides(outside);
                seen.extend(inside);
            }
        }
    }
    res
}

fn count_sides(set: FxHashSet<[i32; 4]>) -> usize {
    let mut res = 0;
    let mut seen = FxHashSet::default();
    while seen.len() < set.len() {
        for &p in set.iter() {
            if seen.contains(&p) {
                continue;
            }
            let mut queue = VecDeque::from([p]);
            seen.insert(p);
            while let Some([row, col, dr, dc]) = queue.pop_front() {
                for d in [-1, 1] {
                    if dr == 0 {
                        // dc==1 this is a up-down fence
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
        assert_eq!(p1(TEST1), 140);
        assert_eq!(p1(TEST2), 772);
        assert_eq!(p1(TEST3), 1930);
        assert_eq!(p1(INPUT), 1424006);

        assert_eq!(p2(TEST1), 80);
        assert_eq!(p2(TEST2), 436);
        assert_eq!(p2(TEST3), 1206);
        assert_eq!(p2(INPUT), 858684);
    }
}
