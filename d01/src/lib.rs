use std::collections::HashMap;

pub fn p1(s: &str) -> u32 {
    let (mut a, mut b) = parse(s);
    a.sort_unstable();
    b.sort_unstable();
    a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn p2(s: &str) -> u32 {
    let (a, b) = parse(s);
    let b = b.into_iter().fold(HashMap::new(), |mut acc, v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    });
    a.into_iter()
        .map(|k| b.get(&k).map(|v| v * k).unwrap_or(0))
        .sum()
}

fn parse(s: &str) -> (Vec<u32>, Vec<u32>) {
    s.lines()
        .map(|s| {
            let mut it = s.split_ascii_whitespace();
            (
                it.next().and_then(|s| s.parse().ok()).unwrap(),
                it.next().and_then(|s| s.parse().ok()).unwrap(),
            )
        })
        .fold((vec![], vec![]), |(mut v1, mut v2), (a, b)| {
            v1.push(a);
            v2.push(b);
            (v1, v2)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    const TEST: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        debug_assert_eq!(p1(SAMPLE), 11);
        debug_assert_eq!(p1(TEST), 1189304);

        debug_assert_eq!(p2(SAMPLE), 31);
        debug_assert_eq!(p2(TEST), 24349736);
    }
}
