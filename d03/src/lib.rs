use std::sync::LazyLock;

use regex::Regex;

static MRE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
static DRE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d+),(\d)+\)|do\(\)|don't\(\)").unwrap());

pub fn p1(s: &str) -> u64 {
    let mut res = 0;
    for cap in MRE.captures_iter(s) {
        let (_, [a, b]) = cap.extract();
        res += a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap();
    }
    res
}

pub fn p2(s: &str) -> u64 {
    let mut res = 0;
    let mut flag = true;
    for cap in DRE.find_iter(s).map(|cap| cap.as_str()) {
        match cap {
            "do()" => flag = true,
            "don't()" => flag = false,
            _ if flag => res += p1(cap),
            _ => (),
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const TEST: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(SAMPLE1), 161);
        assert_eq!(p1(TEST), 160672468);

        assert_eq!(p2(SAMPLE2), 48);
        assert_eq!(p2(TEST), 84893551);
    }
}
