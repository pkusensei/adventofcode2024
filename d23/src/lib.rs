use std::{collections::VecDeque, iter::once};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

pub fn p1(s: &str) -> usize {
    let (adj, tnames) = build(s);
    let mut total = FxHashSet::default();
    let mut queue: VecDeque<_> = tnames.into_iter().map(|n| (n, vec![n])).collect();
    while let Some((name, mut path)) = queue.pop_front() {
        if path.len() == 3 {
            path.sort_unstable();
            total.insert(path);
            continue;
        }
        for &next in adj[name].iter() {
            let mut temp = path.clone();
            if !temp.contains(&next) && adj[next].contains(path[0]) {
                temp.push(next);
                queue.push_back((next, temp));
            }
        }
    }
    total.len()
}

pub fn p2(s: &str) -> String {
    let (adj, _) = build(s);
    let mut res = FxHashSet::default();
    dfs(
        Default::default(),
        adj.keys().copied().collect(),
        Default::default(),
        &adj,
        &mut res,
    );
    res.into_iter().sorted_unstable().join(",")
}

// Bron-Kerbosch algorithm
fn dfs<'a>(
    curr: FxHashSet<&'a str>,
    mut potential: FxHashSet<&'a str>,
    mut processed: FxHashSet<&'a str>,
    adj: &FxHashMap<&str, FxHashSet<&'a str>>,
    res: &mut FxHashSet<&'a str>,
) {
    if potential.is_empty() && processed.is_empty() {
        if curr.len() > res.len() {
            *res = curr;
        }
        return;
    }
    for node in potential.clone() {
        let neighbors = &adj[node];
        dfs(
            curr.iter().copied().chain(once(node)).collect(),
            potential.intersection(neighbors).copied().collect(),
            processed.intersection(neighbors).copied().collect(),
            adj,
            res,
        );
        potential.remove(node);
        processed.insert(node);
    }
}

fn build(s: &str) -> (FxHashMap<&str, FxHashSet<&str>>, FxHashSet<&str>) {
    let mut adj = FxHashMap::<_, FxHashSet<_>>::default();
    let mut tnames = FxHashSet::default();
    for [a, b] in parse(s) {
        adj.entry(a).or_default().insert(b);
        adj.entry(b).or_default().insert(a);
        if a.starts_with('t') {
            tnames.insert(a);
        }
        if b.starts_with('t') {
            tnames.insert(b);
        }
    }
    (adj, tnames)
}

fn parse(s: &str) -> impl Iterator<Item = [&str; 2]> + '_ {
    s.lines()
        .filter_map(|line| line.split_once('-').map(|(a, b)| [a, b]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(TEST), 7);
        assert_eq!(p1(INPUT), 1411);

        assert_eq!(p2(TEST), "co,de,ka,ta");
        assert_eq!(p2(INPUT), "aq,bn,ch,dt,gu,ow,pk,qy,tv,us,yx,zg,zu");
    }
}
