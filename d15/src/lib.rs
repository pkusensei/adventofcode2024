use std::convert::identity;

use fxhash::{FxHashMap, FxHashSet};
use num_complex::Complex;
use utils::parse_with_lens;

pub fn p1(s: &str) -> i64 {
    let ([walls, mut boxes], mut rob, inst) = parse(s).unwrap();
    for b in inst.bytes().filter(|b| b"<>^v".contains(b)) {
        rob = dfs(rob, dir(b), &walls, &mut boxes).unwrap_or(rob);
    }
    boxes.into_iter().map(|b| b.re + 100 * b.im).sum()
}

pub fn p2(s: &str) -> i64 {
    let (walls, mut boxes, mut rob, inst) = parse2(s).unwrap();
    for b in inst.bytes() {
        if b"<>".contains(&b) {
            rob = left_right(rob, dir(b), &walls, &mut boxes).unwrap_or(rob);
        }
        if b"^v".contains(&b) && up_down(rob, dir(b), &walls, &mut boxes, true).is_some() {
            rob = up_down(rob, dir(b), &walls, &mut boxes, false).unwrap_or(rob);
        }
    }
    let n = boxes.len() / 2;
    let mut xmap = FxHashMap::<_, Vec<_>>::default();
    let mut ydist = vec![0; n];
    for b in boxes.into_iter() {
        xmap.entry(b.1).or_default().push(b.0.re);
        ydist[b.1] = b.0.im;
    }
    let mut res = 0;
    for i in 0..n {
        res += xmap[&i].iter().min().unwrap() + 100 * ydist[i]
    }
    res
}

type Pos = Complex<i64>;

fn dfs(curr: Pos, dir: Pos, walls: &FxHashSet<Pos>, boxes: &mut FxHashSet<Pos>) -> Option<Pos> {
    let next = curr + dir;
    if walls.contains(&next) {
        None
    } else if boxes.contains(&next) {
        if let Some(empty) = dfs(next, dir, walls, boxes) {
            boxes.insert(empty);
            boxes.remove(&next);
            Some(next)
        } else {
            None
        }
    } else {
        Some(next)
    }
}

fn left_right(
    curr: Pos,
    dir: Pos,
    walls: &FxHashSet<Pos>,
    boxes: &mut FxHashMap<Pos, usize>,
) -> Option<Pos> {
    debug_assert!(dir == Complex::ONE || dir == -Complex::ONE);
    let next = curr + dir;
    if walls.contains(&next) {
        None
    } else if let Some(&id) = boxes.get(&next) {
        if let Some(empty) = left_right(next, dir, walls, boxes) {
            boxes.insert(empty, id);
            boxes.remove(&next);
            Some(next)
        } else {
            None
        }
    } else {
        Some(next)
    }
}

fn up_down(
    curr: Pos,
    dir: Pos,
    walls: &FxHashSet<Pos>,
    boxes: &mut FxHashMap<Pos, usize>,
    dry_run: bool,
) -> Option<Pos> {
    debug_assert!(dir == Complex::I || dir == -Complex::I);
    let next = curr + dir;
    if walls.contains(&next) {
        None
    } else if let Some(&id) = boxes.get(&next) {
        step(boxes, next + Complex::ONE, id, next, dir, walls, dry_run)
            .or_else(|| step(boxes, next - Complex::ONE, id, next, dir, walls, dry_run))
    } else {
        Some(next)
    }
}

fn step(
    boxes: &mut FxHashMap<Pos, usize>,
    neighbor: Pos,
    id: usize,
    next: Pos,
    dir: Pos,
    walls: &FxHashSet<Pos>,
    dry_run: bool,
) -> Option<Pos> {
    if boxes.get(&neighbor).is_some_and(|&v| v == id) {
        if let (Some(v1), Some(v2)) = (
            up_down(next, dir, walls, boxes, dry_run),
            up_down(neighbor, dir, walls, boxes, dry_run),
        ) {
            if !dry_run {
                boxes.remove(&next);
                boxes.remove(&neighbor);
                boxes.insert(v1, id);
                boxes.insert(v2, id);
            }
            return Some(next);
        }
    }
    None
}

fn dir(b: u8) -> Pos {
    match b {
        b'<' => -Complex::ONE,
        b'>' => Complex::ONE,
        b'^' => -Complex::I,
        b'v' => Complex::I,
        _ => unreachable!(),
    }
}

fn parse(s: &str) -> Option<([FxHashSet<Pos>; 2], Pos, &str)> {
    let (map, inst) = s.split_once("\n\n")?;
    let (_, it) = parse_with_lens(map, &identity);
    let [mut walls, mut boxes] = [0; 2].map(|_| FxHashSet::default());
    let mut start = Complex::default();
    for ((row, col), b) in it {
        let curr = Complex::new(col as i64, row as i64);
        match b {
            b'@' => start = curr,
            b'#' => {
                walls.insert(curr);
            }
            b'O' => {
                boxes.insert(curr);
            }
            _ => (),
        }
    }
    Some(([walls, boxes], start, inst))
}

fn parse2(s: &str) -> Option<(FxHashSet<Pos>, FxHashMap<Pos, usize>, Pos, &str)> {
    let (map, inst) = s.split_once("\n\n")?;
    let (_, it) = parse_with_lens(map, &identity);
    let mut walls = FxHashSet::default();
    let mut boxes = FxHashMap::default();
    let mut start = Complex::default();
    for ((row, col), b) in it {
        let curr = Complex::new(2 * col as i64, row as i64);
        match b {
            b'@' => start = curr,
            b'#' => {
                walls.insert(curr);
                walls.insert(curr + Complex::ONE);
            }
            b'O' => {
                boxes.insert(curr, boxes.len() / 2);
                boxes.insert(curr + Complex::ONE, boxes.len() / 2);
            }
            _ => (),
        }
    }
    Some((walls, boxes, start, inst))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    const TEST2: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    const INPUT: &str = include_str!("../input.txt");

    const TEST3: &str = r#"#######
#.....#
#.O#..#
#..O@.#
#.....#
#######

<v<<^"#;
    //
    // ..[]##..
    // ...[]...
    // ...@....
    //

    #[test]
    fn it_works() {
        assert_eq!(p1(TEST1), 2028);
        assert_eq!(p1(TEST2), 10092);
        assert_eq!(p1(INPUT), 1499739);

        assert_eq!(p2(TEST2), 9021);
        assert_eq!(p2(TEST3), 509);
        assert_eq!(p2(INPUT), 1522215);
    }
}
