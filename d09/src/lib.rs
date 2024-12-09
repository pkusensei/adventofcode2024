use std::iter;

pub fn p1(s: &str) -> usize {
    let mut nums = parse(s);
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        while nums[left].is_some() {
            left += 1;
        }
        while nums[right].is_none() {
            right -= 1;
        }
        if left < right {
            nums.swap(left, right);
        }
        left += 1;
        right -= 1;
    }
    nums.into_iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.map(|v| v * i))
        .sum()
}

pub fn p2(s: &str) -> usize {
    let mut nums = parse(s);
    let mut indices: Vec<(usize, Option<usize>)> = nums.clone().into_iter().enumerate().collect();
    let mut skip = 0;
    loop {
        let Some((right, len)) = indices
            .chunk_by(|a, b| a.1 == b.1)
            .filter(|p| p[0].1.is_some())
            .rev()
            .nth(skip)
            .map(|p| (p[0].0, p.len()))
        else {
            break;
        };
        if right == 0 {
            break;
        }
        let Some(left) = indices
            .chunk_by(|a, b| a.1 == b.1)
            .find(|p| p.len() >= len && p[0].1.is_none() && p[0].0 < right)
            .map(|p| p[0].0)
        else {
            skip += 1;
            continue;
        };
        for i in 0..len {
            nums.swap(left + i, right + i);
            indices[left + i].1 = indices[right + i].1;
            indices[right + i].1 = None;
        }
    }
    nums.into_iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.map(|v| v * i))
        .sum()
}

fn parse(s: &str) -> Vec<Option<usize>> {
    let mut is_file = true;
    let mut id = 0;
    let mut res = vec![];
    for b in s.trim().bytes() {
        let len = usize::from(b - b'0');
        if is_file {
            res.extend(iter::repeat_n(Some(id), len));
            id += 1;
        } else {
            res.extend(iter::repeat_n(None, len));
        }
        is_file = !is_file;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "2333133121414131402";
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(TEST), 1928);
        assert_eq!(p1(INPUT), 6320029754031);

        assert_eq!(p2(TEST), 2858);
        assert_eq!(p2(INPUT), 6347435485773);
    }
}
