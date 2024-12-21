use std::{
    iter::{once, repeat_n},
    sync::LazyLock,
};

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
static NUMMAP: LazyLock<FxHashMap<u8, [i8; 2]>> = LazyLock::new(|| build_map(&NUMPAD));
static KEYMAP: LazyLock<FxHashMap<u8, [i8; 2]>> = LazyLock::new(|| build_map(&KEYPAD));

fn solve(s: &str, depth: i8) -> usize {
    let mut res = 0;
    let mut memo = FxHashMap::default();
    for line in s.lines() {
        let len = dfs(line.as_bytes().to_vec(), depth, &NUMPAD, &NUMMAP, &mut memo);
        let val = line[..line.len() - 1].parse().unwrap_or(1);
        res += len * val;
    }
    res
}

fn dfs(
    seq: Vec<u8>,
    depth: i8,
    pad: &[&[u8; 3]],
    map: &FxHashMap<u8, [i8; 2]>,
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
        res += get_paths(pad, map, a, b)
            .into_iter()
            .map(|p| dfs(p, depth - 1, &KEYPAD, &KEYMAP, memo))
            .min()
            .unwrap_or(0);
        a = b;
    }
    memo.insert((seq, depth), res);
    res
}

fn get_paths(pad: &[&[u8; 3]], map: &FxHashMap<u8, [i8; 2]>, k1: u8, k2: u8) -> Vec<Vec<u8>> {
    if k1 == k2 {
        return vec![vec![b'A']];
    }
    let [r1, c1] = map[&k1];
    let [r2, c2] = map[&k2];
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

fn build_map(pad: &[&[u8; 3]]) -> FxHashMap<u8, [i8; 2]> {
    let mut map = FxHashMap::default();
    for (y, line) in pad.iter().enumerate() {
        for (x, &b) in line.iter().enumerate() {
            map.insert(b, [y as _, x as _]);
        }
    }
    map
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
