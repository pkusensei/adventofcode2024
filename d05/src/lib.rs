use std::collections::{HashMap, HashSet};

pub fn p1(s: &str) -> u64 {
    let (rules, updates) = parse(s);
    let rules = rules
        .into_iter()
        .fold(HashMap::<_, HashSet<_>>::new(), |mut acc, [a, b]| {
            acc.entry(a).or_default().insert(b);
            acc
        });
    updates
        .into_iter()
        .filter_map(|up| check(&rules, &up))
        .sum()
}

pub fn p2(s: &str) -> u64 {
    let (rules, updates) = parse(s);
    let rules = rules
        .into_iter()
        .fold(HashMap::<_, HashSet<_>>::new(), |mut acc, [a, b]| {
            acc.entry(a).or_default().insert(b);
            acc
        });
    updates
        .into_iter()
        .filter_map(|up| build(&up, &rules))
        .sum()
}

fn build(up: &[u64], rules: &HashMap<u64, HashSet<u64>>) -> Option<u64> {
    let mut v = Vec::with_capacity(up.len());
    for num in up.iter() {
        if let Some(i) = v
            .iter()
            .position(|v| rules.get(num).is_some_and(|s| s.contains(v)))
        {
            v.insert(i, *num);
        } else {
            v.push(*num);
        }
    }
    if up == v {
        None
    } else {
        let n = up.len();
        Some(v[n / 2])
    }
}

fn check(rules: &HashMap<u64, HashSet<u64>>, update: &[u64]) -> Option<u64> {
    let n = update.len();
    let mut mid = 0;
    for (i, front) in update.iter().enumerate() {
        if i == n / 2 {
            mid = *front;
        }
        if update[1 + i..]
            .iter()
            .any(|back| rules.get(front).is_none_or(|v| !v.contains(back)))
        {
            return None;
        }
    }
    Some(mid)
}

fn parse(s: &str) -> (Vec<[u64; 2]>, Vec<Vec<u64>>) {
    let mut flag = false;
    let mut rules = vec![];
    let mut updates = vec![];
    for line in s.lines() {
        if line.is_empty() {
            flag = true;
            continue;
        }
        if !flag {
            let mut it = line.split('|');
            let r = [it.next(), it.next()].map(|v| v.and_then(|s| s.parse().ok()).unwrap_or(0));
            rules.push(r);
        } else {
            let v = line.split(',').map(|s| s.parse().unwrap_or(0)).collect();
            updates.push(v);
        }
    }
    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    const TEST: &str = include_str!("../input.txt");

    #[test]
    fn t1() {
        assert_eq!(p1(SAMPLE), 143);
        assert_eq!(p1(TEST), 5208);
    }

    #[test]
    fn t2() {
        assert_eq!(p2(SAMPLE), 123);
        assert_eq!(p2(TEST), 6732);
    }
}
