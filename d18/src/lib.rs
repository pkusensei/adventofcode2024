use std::{collections::VecDeque, iter::once};

use fxhash::FxHashSet;
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
            if (0..=max).contains(&next.re) && (0..=max).contains(&next.im) && nogo.insert(next) {
                queue.push_back((next, 1 + dist));
            }
        }
    }
    0
}

fn step(curr: Complex<i16>) -> impl Iterator<Item = Complex<i16>> {
    once(curr + Complex::ONE)
        .chain(once(curr - Complex::ONE))
        .chain(once(curr + Complex::I))
        .chain(once(curr - Complex::I))
}

fn parse(s: &str) -> impl Iterator<Item = Complex<i16>> + '_ {
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
    }
}
