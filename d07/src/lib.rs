pub fn solve(s: &str, p2: bool) -> u64 {
    let mut res = 0;
    let f = if p2 { dfs2 } else { dfs1 };
    for line in parse(s) {
        if line.len() >= 2 {
            res += f(line[0], line[1], &line[2..]).unwrap_or(0);
        }
    }
    res
}

fn dfs1(target: u64, curr: u64, nums: &[u64]) -> Option<u64> {
    match nums {
        [] if curr == target => Some(target),
        [head, tail @ ..] if curr <= target => {
            dfs1(target, curr + head, tail).or_else(|| dfs1(target, curr * head, tail))
        }
        _ => None,
    }
}

fn dfs2(target: u64, curr: u64, nums: &[u64]) -> Option<u64> {
    match nums {
        [] if curr == target => Some(target),
        [head, tail @ ..] if curr <= target => {
            let p = 1 + head.ilog10();
            let num = curr * 10u64.pow(p) + head;
            dfs2(target, num, tail)
                .or_else(|| dfs2(target, curr + head, tail))
                .or_else(|| dfs2(target, curr * head, tail))
        }
        _ => None,
    }
}

fn parse(s: &str) -> impl Iterator<Item = Vec<u64>> + '_ {
    s.lines().map(|line| {
        line.split(|c: char| !c.is_ascii_alphanumeric())
            .filter_map(|v| v.parse().ok())
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    const TEST: &str = include_str!("../input.txt");

    #[test]
    fn test() {
        assert_eq!(solve(SAMPLE, false), 3749);
        assert_eq!(solve(TEST, false), 1289579105366);

        assert_eq!(solve(SAMPLE, true), 11387);
        assert_eq!(solve(TEST, true), 92148721834692);
    }
}
