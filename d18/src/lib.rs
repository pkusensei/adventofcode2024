use std::{collections::VecDeque, iter::once};

use fxhash::FxHashSet;
use itertools::Itertools;
use num_complex::Complex;

pub fn p1(s: &str, test: bool) -> u64 {
    let (max, take) = if test { (6, 12) } else { (70, 1024) };
    let it = parse(s);
    let mut queue = VecDeque::from([(Complex::default(), 0)]);
    let mut nogo: FxHashSet<_> = it.take(take).collect();
    nogo.insert(Complex::default());
    while let Some((curr, dist)) = queue.pop_front() {
        if curr == Complex::new(max, max) {
            return dist;
        }
        for next in step(curr) {
            if is_open(max, &nogo, next) {
                nogo.insert(next);
                queue.push_back((next, 1 + dist));
            }
        }
    }
    0
}

pub fn p2(s: &str, test: bool) -> String {
    let max = if test { 6 } else { 70 };
    let it = parse(s);
    let n = ((max + 1) * (max + 1)) as usize;
    let mut dsu = DSU::new(n);
    let mut blocks: FxHashSet<_> = it.clone().collect();
    for c in (0..=max)
        .cartesian_product(0..=max)
        .map(|(x, y)| Complex::new(x, y))
        .filter(|c| is_open(max, &blocks, *c))
    {
        for n in step(c).filter(|c| is_open(max, &blocks, *c)) {
            dsu.union(id_of(c, max), id_of(n, max));
        }
    }
    let start = 0;
    let goal = ((max + 1) * (max + 1)) as usize - 1;
    for block in it.rev() {
        blocks.remove(&block);
        for n in step(block).filter(|c| is_open(max, &blocks, *c)) {
            dsu.union(id_of(block, max), id_of(n, max));
        }
        if dsu.find(start) == dsu.find(goal) {
            return format!("{},{}", block.re, block.im);
        }
    }
    "".into()
}

fn id_of(c: Complex<i16>, max: i16) -> usize {
    (c.im * (max + 1) + c.re) as _
}

fn is_open(max: i16, nogo: &FxHashSet<Complex<i16>>, c: Complex<i16>) -> bool {
    (0..=max).contains(&c.re) && (0..=max).contains(&c.im) && !nogo.contains(&c)
}

#[derive(Debug, Clone)]
struct DSU {
    parent: Vec<usize>,
    rank: Vec<i16>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
    }
}

fn step(curr: Complex<i16>) -> impl Iterator<Item = Complex<i16>> {
    once(curr + Complex::ONE)
        .chain(once(curr - Complex::ONE))
        .chain(once(curr + Complex::I))
        .chain(once(curr - Complex::I))
}

fn parse(s: &str) -> impl DoubleEndedIterator<Item = Complex<i16>> + Clone + '_ {
    s.lines().map(|line| {
        let mut it = line.split(',');
        Complex {
            re: it.next().and_then(|v| v.parse::<i16>().ok()).unwrap(),
            im: it.next().and_then(|v| v.parse::<i16>().ok()).unwrap(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(TEST, true), 22);
        assert_eq!(p1(INPUT, false), 260);

        assert_eq!(p2(TEST, true), "6,1");
        assert_eq!(p2(INPUT, false), "24,48");
    }
}
