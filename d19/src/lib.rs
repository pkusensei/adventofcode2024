use fxhash::FxHashMap;

pub fn solve(s: &str) -> [usize; 2] {
    let (pats, lines) = parse(s);
    let [mut p1, mut p2] = [0; 2];
    let mut memo = FxHashMap::default();
    for line in lines {
        let t = dfs(line, &pats, &mut memo);
        p1 += usize::from(t > 0);
        p2 += t;
    }
    [p1, p2]
}

fn dfs<'a>(line: &'a str, pats: &[&str], memo: &mut FxHashMap<&'a str, usize>) -> usize {
    if line.is_empty() {
        return 1;
    }
    if let Some(&v) = memo.get(line) {
        return v;
    }
    let mut res = 0;
    for &p in pats.iter() {
        if let Some(v) = line.strip_prefix(p).map(|tail| dfs(tail, pats, memo)) {
            res += v;
        }
    }
    memo.insert(line, res);
    res
}

fn parse(s: &str) -> (Vec<&str>, Vec<&str>) {
    let (pats, designs) = s.split_once("\n\n").unwrap();
    (pats.split(", ").collect(), designs.lines().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(solve(TEST), [6, 16]);
        assert_eq!(solve(INPUT), [255, 621820080273474]);
    }
}
