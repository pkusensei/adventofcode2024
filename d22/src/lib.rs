use std::collections::VecDeque;

use fxhash::FxHashMap;

pub fn solve(s: &str) -> [i64; 2] {
    let mut p1 = 0;
    let mut p2 = FxHashMap::default();
    for n in parse(s) {
        let (sum, curr) = process(n);
        p1 += sum;
        for (k, v) in curr {
            *p2.entry(k).or_insert(0) += v;
        }
    }
    [p1, p2.into_values().max().unwrap_or(0)]
}

fn process(mut num: i64) -> (i64, FxHashMap<Vec<i64>, i64>) {
    let mut digit = num % 10;
    let mut seq = VecDeque::with_capacity(5);
    let mut curr = FxHashMap::default();
    for _ in 0..2000 {
        num = pseudo(num);
        seq.push_back(num % 10 - digit);
        digit = num % 10;
        while seq.len() > 4 {
            seq.pop_front();
        }
        if seq.len() == 4 && digit > 0 {
            curr.entry(Vec::from(seq.clone())).or_insert(digit);
        }
    }
    (num, curr)
}

const fn pseudo(mut n: i64) -> i64 {
    const PRUNE: i64 = 0xFFFFFF;

    n ^= n << 6;
    n &= PRUNE;
    n ^= n >> 5;
    n &= PRUNE;
    n ^= n * 2048;
    n & PRUNE
}

fn parse(s: &str) -> impl Iterator<Item = i64> + '_ {
    s.lines().filter_map(|line| line.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"1
10
100
2024"#;

    const TEST2: &str = r#"1
2
3
2024"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solve(TEST)[0], 37327623);
        assert_eq!(solve(TEST2)[1], 23);

        let [p1, p2] = solve(INPUT);
        assert_eq!(p1, 16953639210);
        assert_eq!(p2, 1863);
    }
}
