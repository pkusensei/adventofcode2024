use std::iter::{once, repeat_n};

use fxhash::FxHashMap;

pub fn p1(s: &str) -> usize {
    solve(s, 3)
}

pub fn p2(s: &str) -> usize {
    solve(s, 26)
}

const EMPTY: u8 = b'#';
const NUMPAD: [&[u8; 3]; 4] = [b"789", b"456", b"123", b"#0A"];
const KEYPAD: [&[u8; 3]; 2] = [b"#^A", b"<v>"];

fn solve(s: &str, depth: i8) -> usize {
    let mut res = 0;
    let mut memo = FxHashMap::default();
    for line in s.lines() {
        let len = dfs(line.as_bytes().to_vec(), depth, &NUMPAD, &mut memo);
        let val = line[..line.len() - 1].parse().unwrap_or(1);
        res += len * val;
    }
    res
}

fn dfs(
    seq: Vec<u8>,
    depth: i8,
    pad: &[&[u8; 3]],
    memo: &mut FxHashMap<(Vec<u8>, i8), usize>,
) -> usize {
    if depth == 0 {
        return seq.len();
    }
    if let Some(&v) = memo.get(&(seq.clone(), depth)) {
        return v;
    }
    let mut res = 0;
    let mut a = b'A';
    for &b in seq.iter() {
        res += get_paths(pad, a, b)
            .into_iter()
            .map(|p| dfs(p, depth - 1, &KEYPAD, memo))
            .min()
            .unwrap_or(0);
        a = b;
    }
    memo.insert((seq, depth), res);
    res
}

fn get_paths(pad: &[&[u8; 3]], k1: u8, k2: u8) -> Vec<Vec<u8>> {
    if k1 == k2 {
        return vec![vec![b'A']];
    }
    let [r1, c1] = locate(k1, pad);
    let [r2, c2] = locate(k2, pad);
    let dr = r2 - r1;
    let dc = c2 - c1;
    let rows = if dr >= 0 {
        repeat_n(b'v', dr as usize)
    } else {
        repeat_n(b'^', (-dr) as usize)
    };
    let cols = if dc >= 0 {
        repeat_n(b'>', dc as usize)
    } else {
        repeat_n(b'<', (-dc) as usize)
    };
    if dr == 0 {
        vec![cols.chain(once(b'A')).collect()]
    } else if dc == 0 {
        vec![rows.chain(once(b'A')).collect()]
    } else if pad[r1 as usize][c2 as usize] == EMPTY {
        vec![rows.chain(cols).chain(once(b'A')).collect()]
    } else if pad[r2 as usize][c1 as usize] == EMPTY {
        vec![cols.chain(rows).chain(once(b'A')).collect()]
    } else {
        vec![
            rows.clone().chain(cols.clone()).chain(once(b'A')).collect(),
            cols.chain(rows).chain(once(b'A')).collect(),
        ]
    }
}

fn locate(b: u8, pad: &[&[u8; 3]]) -> [i8; 2] {
    let [mut row, mut col] = [0; 2];
    for (y, r) in pad.iter().enumerate() {
        for (x, &v) in r.iter().enumerate() {
            if v == b {
                row = y as _;
                col = x as _;
            }
        }
    }
    [row, col]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"029A
980A
179A
456A
379A"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(TEST), 126384);
        assert_eq!(p1(INPUT), 206798);

        assert_eq!(p2(TEST), 154115708116294);
        assert_eq!(p2(INPUT), 251508572750680);
    }
}
