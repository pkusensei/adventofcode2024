use std::collections::VecDeque;

use fxhash::FxHashSet;
use utils::{deltas, parse_with_lens};

pub fn p1(s: &str) -> i64 {
    let (grid, heads) = build(s);
    heads.into_iter().map(|h| bfs(&grid, h)).sum()
}

pub fn p2(s: &str) -> i64 {
    let (grid, heads) = build(s);
    let mut memo = vec![vec![-1; grid[0].len()]; grid.len()];
    heads.into_iter().map(|h| dfs(&grid, h, 0, &mut memo)).sum()
}

fn build(s: &str) -> (Vec<Vec<u8>>, Vec<[usize; 2]>) {
    let ((rows, cols), it) = parse_with_lens(s, &|b| b - b'0');
    let mut grid = vec![vec![0; cols]; rows];
    let mut heads = vec![];
    for ((row, col), b) in it {
        grid[row][col] = b;
        if b == 0 {
            heads.push([row, col]);
        }
    }
    (grid, heads)
}

fn bfs(grid: &[Vec<u8>], start: [usize; 2]) -> i64 {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut seen = FxHashSet::default();
    seen.insert(start);
    let mut res = 0;
    while let Some(([row, col], val)) = queue.pop_front() {
        if grid[row][col] == 9 {
            res += 1;
            continue;
        }
        for ((nr, nc), _) in deltas(row, col) {
            if grid
                .get(nr)
                .is_some_and(|r| r.get(nc).is_some_and(|&v| val + 1 == v))
                && seen.insert([nr, nc])
            {
                queue.push_back(([nr, nc], 1 + val));
            }
        }
    }
    res
}

fn dfs(grid: &[Vec<u8>], [row, col]: [usize; 2], val: u8, memo: &mut [Vec<i64>]) -> i64 {
    if grid[row][col] == 9 {
        return 1;
    }
    if memo[row][col] > -1 {
        return memo[row][col];
    }
    let mut res = 0;
    for ((nr, nc), _) in deltas(row, col) {
        if grid
            .get(nr)
            .is_some_and(|r| r.get(nc).is_some_and(|&v| val + 1 == v))
        {
            res += dfs(grid, [nr, nc], 1 + val, memo);
        }
    }
    memo[row][col] = res;
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(TEST), 36);
        assert_eq!(p1(INPUT), 535);

        assert_eq!(p2(TEST), 81);
        assert_eq!(p2(INPUT), 1186);
    }
}
