use fxhash::FxHashMap;

pub fn solve(s: &str) -> (u64, u64) {
    let mut count = s
        .split_whitespace()
        .map(|v| v.parse().unwrap_or(0u64))
        .fold(FxHashMap::default(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
    let mut p1 = 0;
    for i in 0..75 {
        count = step(count);
        if i == 24 {
            p1 = count.values().sum();
        }
    }
    let p2 = count.into_values().sum();
    (p1, p2)
}

fn step(map: FxHashMap<u64, u64>) -> FxHashMap<u64, u64> {
    let mut res = FxHashMap::default();
    for (key, val) in map.into_iter() {
        match key {
            0 => {
                *res.entry(1).or_insert(0) += val;
            }
            _ if (key.ilog10() & 1) == 1 => {
                let len = 1 + key.ilog10();
                let left = key / 10u64.pow(len / 2);
                let right = key % 10u64.pow(len / 2);
                *res.entry(left).or_insert(0) += val;
                *res.entry(right).or_insert(0) += val;
            }
            _ => {
                *res.entry(key * 2024).or_insert(0) += val;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "125 17";

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        let (p1, p2) = solve(TEST);
        assert_eq!(p1, 55312);
        assert_eq!(p2, 65601038650482);

        let (p1, p2) = solve(INPUT);
        assert_eq!(p1, 233875);
        assert_eq!(p2, 277444936413293);
    }
}
