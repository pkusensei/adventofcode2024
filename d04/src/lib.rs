use std::convert::identity;

use utils::*;

pub fn p1(s: &str) -> u64 {
    let grid: Vec<_> = s.lines().map(|line| line.as_bytes()).collect();
    let (_, it) = parse_with_lens(s, &identity);
    let mut res = 0;
    for ((y, x), b) in it {
        if b == b'X' {
            for dir in DELTAS8 {
                res += dfs(&grid, y as _, x as _, dir[0], dir[1], b'X');
            }
        }
    }
    res
}

pub fn p2(s: &str) -> u64 {
    let grid: Vec<_> = s.lines().map(|line| line.as_bytes()).collect();
    let ((rows, cols), it) = parse_with_lens(s, &identity);
    let mut res = 0;
    for ((y, x), b) in it {
        if check(b, rows, y, cols, x, &grid) {
            res += 1;
        }
    }
    res
}

fn dfs(grid: &[&[u8]], cr: i32, cc: i32, dx: i32, dy: i32, curr: u8) -> u64 {
    if grid[cr as usize][cc as usize] != curr {
        return 0;
    }
    if curr == b'S' {
        return 1;
    }
    let next = match curr {
        b'X' => b'M',
        b'M' => b'A',
        b'A' => b'S',
        _ => return 0,
    };
    let ny = cr + dy;
    let nx = cc + dx;
    if nx >= 0
        && ny >= 0
        && grid
            .get(ny as usize)
            .is_some_and(|row| row.get(nx as usize).is_some())
    {
        dfs(grid, ny, nx, dx, dy, next)
    } else {
        0
    }
}

fn check(b: u8, rows: usize, y: usize, cols: usize, x: usize, grid: &[&[u8]]) -> bool {
    b == b'A'
        && (1..rows - 1).contains(&y)
        && (1..cols - 1).contains(&x)
        && ((grid[y - 1][x - 1] == b'M' && grid[y + 1][x + 1] == b'S')
            || (grid[y - 1][x - 1] == b'S' && grid[y + 1][x + 1] == b'M'))
        && ((grid[y + 1][x - 1] == b'M' && grid[y - 1][x + 1] == b'S')
            || (grid[y + 1][x - 1] == b'S' && grid[y - 1][x + 1] == b'M'))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    const TEST: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(SAMPLE), 18);
        assert_eq!(p1(TEST), 2462);

        assert_eq!(p2(SAMPLE), 9);
        assert_eq!(p2(TEST), 1877);
    }
}
