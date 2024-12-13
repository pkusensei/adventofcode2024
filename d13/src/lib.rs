pub fn p1(s: &str) -> i64 {
    parse(s)
        .filter_map(|v| solve(v, false))
        .map(|[a, b]| 3 * a + b)
        .sum()
}

pub fn p2(s: &str) -> i64 {
    parse(s)
        .filter_map(|v| solve(v, true))
        .map(|[a, b]| 3 * a + b)
        .sum()
}

fn solve(nums: Vec<i64>, p2: bool) -> Option<[i64; 2]> {
    let [x1, y1, x2, y2, mut n1, mut n2] = [nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]];
    if p2 {
        n1 += 10_000_000_000_000;
        n2 += 10_000_000_000_000;
    }
    // x1*a + x2*b == n1
    // y1*a + y2*b == n2
    // x1*y2*a + x2*y2*b == n1*y2
    // x2*y1*a + x2*y2*b == n2*x2
    // (x1*y2 - x2*y1) *a == n1*y2 - n2*x2
    let right = n1 * y2 - n2 * x2;
    let left = x1 * y2 - x2 * y1;
    if right % left == 0 {
        let a = right / left;
        if (n1 - x1 * a) % x2 == 0 {
            let b = (n1 - x1 * a) / x2;
            if p2 || (0..=100).contains(&a) && (0..=100).contains(&b) {
                return Some([a, b]);
            }
        }
    }
    None
}

fn parse(s: &str) -> impl Iterator<Item = Vec<i64>> + '_ {
    s.split("Button A:").filter_map(|s| {
        if !s.is_empty() {
            let v: Vec<i64> = s
                .trim()
                .split(|c: char| !c.is_ascii_digit())
                .filter_map(|v| v.parse().ok())
                .collect();
            Some(v)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(TEST), 480);
        assert_eq!(p1(INPUT), 29711);

        assert_eq!(p2(TEST), 875318608908);
        assert_eq!(p2(INPUT), 94955433618919);
    }
}
