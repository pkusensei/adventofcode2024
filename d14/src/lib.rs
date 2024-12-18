use std::{str, sync::LazyLock, u64};

use regex::Regex;

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"p=(-*\d+),(-*\d+) v=(-*\d+),(-*\d+)"#).unwrap());

pub fn p1(s: &str, is_test: bool) -> u64 {
    let [xmax, ymax] = if is_test { [11, 7] } else { [101, 103] };
    let [mut a, mut b, mut c, mut d] = [0; 4];
    for [mut x, mut y, dx, dy] in parse(s) {
        x += 100 * dx;
        y += 100 * dy;
        x = x.rem_euclid(xmax);
        y = y.rem_euclid(ymax);
        if (0..xmax / 2).contains(&x) && (0..ymax / 2).contains(&y) {
            a += 1;
        }
        if (1 + xmax / 2..xmax).contains(&x) && (0..ymax / 2).contains(&y) {
            b += 1;
        }
        if (0..xmax / 2).contains(&x) && (1 + ymax / 2..ymax).contains(&y) {
            c += 1;
        }
        if (1 + xmax / 2..xmax).contains(&x) && (1 + ymax / 2..ymax).contains(&y) {
            d += 1;
        }
    }
    a * b * c * d
}

pub fn p2(s: &str) -> i32 {
    let [xmax, ymax] = [101, 103];
    let mut points: Vec<_> = parse(s).collect();
    let mut safety = u64::MAX;
    let mut res = 0;
    for i in 0..xmax * ymax {
        let [mut a, mut b, mut c, mut d] = [0; 4];
        for p in points.iter_mut() {
            p[0] += p[2];
            p[1] += p[3];
            p[0] = p[0].rem_euclid(xmax);
            p[1] = p[1].rem_euclid(ymax);
            let [x, y] = [p[0], p[1]];
            if (0..xmax / 2).contains(&x) && (0..ymax / 2).contains(&y) {
                a += 1;
            }
            if (1 + xmax / 2..xmax).contains(&x) && (0..ymax / 2).contains(&y) {
                b += 1;
            }
            if (0..xmax / 2).contains(&x) && (1 + ymax / 2..ymax).contains(&y) {
                c += 1;
            }
            if (1 + xmax / 2..xmax).contains(&x) && (1 + ymax / 2..ymax).contains(&y) {
                d += 1;
            }
        }
        let temp = a * b * c * d;
        if temp < safety {
            safety = temp;
            res = 1 + i;
        }
    }
    res
}

fn parse(s: &str) -> impl Iterator<Item = [i32; 4]> + '_ {
    s.lines().map(|line| {
        let (_, v) = RE.captures(line).unwrap().extract();
        v.map(|n| n.parse().unwrap())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn t1() {
        assert_eq!(p1(TEST, true), 12);
        assert_eq!(p1(INPUT, false), 224438715);
    }

    #[test]
    fn t2() {
        assert_eq!(p2(INPUT), 7603); // 7603
    }
}
