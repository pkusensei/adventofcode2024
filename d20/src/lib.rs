use std::{collections::VecDeque, convert::identity, iter::once};

use fxhash::{FxHashMap, FxHashSet};
use num_complex::Complex;
use utils::parse_with_lens;

pub fn solve(s: &str, sec: i32) -> [i32; 2] {
    let (walls, [start, end]) = parse(s);
    let path = bfs(&walls, [start, end]);
    let [mut p1, mut p2] = [0; 2];
    for (&k1, &v1) in path.iter() {
        for cheat in step(k1, 2) {
            if path.get(&cheat).is_some_and(|&v| v - v1 - 2 >= sec) {
                p1 += 1;
            }
        }
        for (&k2, &v2) in path.iter().filter(|(k, _)| **k != k1) {
            let dist = k1.re.abs_diff(k2.re) + k1.im.abs_diff(k2.im);
            if dist <= 20 && v1 + i32::from(dist) + sec <= v2 {
                p2 += 1
            }
        }
    }
    [p1, p2]
}

type Pos = Complex<i16>;

fn bfs(walls: &FxHashSet<Pos>, [start, end]: [Pos; 2]) -> FxHashMap<Pos, i32> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut dists = FxHashMap::default();
    dists.insert(start, 0);
    while let Some((curr, dist)) = queue.pop_front() {
        if curr == end {
            break;
        }
        for next in step(curr, 1) {
            if !walls.contains(&next) && !dists.contains_key(&next) {
                dists.insert(next, 1 + dist);
                queue.push_back((next, 1 + dist));
            }
        }
    }
    dists
}

fn step(c: Pos, n: i16) -> impl Iterator<Item = Pos> {
    once(c + n * Pos::ONE)
        .chain(once(c - n * Pos::ONE))
        .chain(once(c + n * Pos::I))
        .chain(once(c - n * Pos::I))
}

fn parse(s: &str) -> (FxHashSet<Pos>, [Pos; 2]) {
    let (_, it) = parse_with_lens(s, &identity);
    let [mut start, mut end] = [Pos::default(); 2];
    let mut walls = FxHashSet::default();
    for ((row, col), b) in it {
        let curr = Pos::new(col as i16, row as i16);
        match b {
            b'S' => start = curr,
            b'E' => end = curr,
            b'#' => {
                walls.insert(curr);
            }
            _ => (),
        }
    }
    (walls, [start, end])
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        let [p1, p2] = solve(TEST, 20);
        assert_eq!(p1, 5);
        assert_eq!(p2, 1449); // no idea about this number

        let [p1, p2] = solve(INPUT, 100);
        assert_eq!(p1, 1393);
        assert_eq!(p2, 990096);
    }
}
