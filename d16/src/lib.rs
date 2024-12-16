use std::{
    collections::{BinaryHeap, VecDeque},
    convert::identity,
    iter,
};

use fxhash::{FxHashMap, FxHashSet};
use num_complex::Complex;
use utils::parse_with_lens;

pub fn solve(s: &str) -> [u32; 2] {
    let (walls, [start, goal]) = parse(s);
    let (p1, prev, queue) = dijkstra(walls, start, goal);
    let p2 = bfs(prev, queue);
    [p1, p2 as _]
}

type Pos = Complex<i16>;

fn dijkstra(
    walls: FxHashSet<Pos>,
    start: Pos,
    goal: Pos,
) -> (
    u32,
    FxHashMap<[Pos; 2], FxHashSet<[Pos; 2]>>,
    VecDeque<[Pos; 2]>,
) {
    let state = State::new(start);
    let mut dists = FxHashMap::default();
    let mut heap = BinaryHeap::from([state]);
    let mut prev = FxHashMap::<_, FxHashSet<_>>::default();
    let mut p1 = u32::MAX;
    let mut queue = VecDeque::new();
    while let Some(state) = heap.pop() {
        if dists
            .get(&[state.pos, state.dir])
            .is_some_and(|&v| v < state.score)
        {
            continue;
        }
        if state.score > p1 {
            break;
        }
        if state.pos == goal {
            p1 = p1.min(state.score);
            queue.push_back([state.pos, state.dir]);
            continue;
        }
        for ns in state.next() {
            if !walls.contains(&ns.pos) {
                if dists.get(&[ns.pos, ns.dir]).is_none_or(|&v| v > ns.score) {
                    heap.push(ns);
                    dists.insert([ns.pos, ns.dir], ns.score);
                    let mut set = FxHashSet::default();
                    set.insert([state.pos, state.dir]);
                    prev.insert([ns.pos, ns.dir], set);
                } else if dists.get(&[ns.pos, ns.dir]).is_some_and(|&v| v == ns.score)
                    && ns.pos != state.pos
                {
                    prev.entry([ns.pos, ns.dir])
                        .or_default()
                        .insert([state.pos, state.dir]);
                }
            }
        }
    }
    (p1, prev, queue)
}

fn bfs(prev: FxHashMap<[Pos; 2], FxHashSet<[Pos; 2]>>, mut queue: VecDeque<[Pos; 2]>) -> usize {
    let mut path = FxHashSet::default();
    let mut seen = FxHashSet::default();
    while let Some(node) = queue.pop_front() {
        let Some(s) = prev.get(&node) else {
            continue;
        };
        if !seen.insert(node) {
            continue;
        }
        for &n in s {
            path.insert(n[0]);
            queue.push_back(n);
        }
    }
    1 + path.len() // +1 to count in start
}

fn parse(s: &str) -> (FxHashSet<Pos>, [Pos; 2]) {
    let (_, it) = parse_with_lens(s, &identity);
    let mut walls = FxHashSet::default();
    let [mut start, mut goal] = [Complex::default(); 2];
    for ((row, col), b) in it {
        let curr = Complex::new(col as i16, row as i16);
        match b {
            b'S' => start = curr,
            b'E' => goal = curr,
            b'#' => {
                walls.insert(curr);
            }
            _ => (),
        }
    }
    (walls, [start, goal])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    pos: Pos,
    dir: Pos,
    score: u32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl State {
    fn new(pos: Pos) -> Self {
        Self {
            pos,
            dir: Complex::ONE,
            score: 0,
        }
    }

    fn next(self) -> impl Iterator<Item = Self> {
        iter::once(Self {
            pos: self.pos + self.dir,
            dir: self.dir,
            score: 1 + self.score,
        })
        .chain(iter::once(Self {
            pos: self.pos,
            dir: self.dir * Complex::I,
            score: 1000 + self.score,
        }))
        .chain(iter::once(Self {
            pos: self.pos,
            dir: self.dir * (-Complex::I),
            score: 1000 + self.score,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const TEST2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        fn run(s: &str, n1: u32, n2: u32) {
            let [p1, p2] = solve(s);
            assert_eq!(p1, n1);
            assert_eq!(p2, n2);
        }
        run(TEST, 7036, 45);
        run(TEST2, 11048, 64);
        run(INPUT, 65436, 489);
    }
}
