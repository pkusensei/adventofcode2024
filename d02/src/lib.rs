use std::cmp::Reverse;

pub fn p1(s: &str) -> usize {
    parse(s)
        .filter(|v| {
            (v.is_sorted() || v.is_sorted_by_key(Reverse))
                && v.windows(2).all(|w| (1..=3).contains(&w[0].abs_diff(w[1])))
        })
        .count()
}

pub fn p2(s: &str) -> usize {
    parse(s).filter(|v| v.len() - max_len(v) <= 1).count()
}

fn max_len(nums: &[i32]) -> usize {
    let n = nums.len();
    let mut inc = vec![1; n];
    let mut dec = inc.clone();
    for i1 in 1..n {
        for i2 in 0..i1 {
            if (1..=3).contains(&nums[i1].abs_diff(nums[i2])) {
                if nums[i1] > nums[i2] {
                    inc[i1] = inc[i1].max(1 + inc[i2]);
                } else {
                    dec[i1] = dec[i1].max(1 + dec[i2]);
                }
            }
        }
    }
    inc.into_iter().chain(dec).max().unwrap()
}

fn parse(s: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    s.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    const TEST: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(SAMPLE), 2);
        assert_eq!(p1(TEST), 624);

        assert_eq!(p2(SAMPLE), 4);
        assert_eq!(p2(TEST), 658);
    }
}
