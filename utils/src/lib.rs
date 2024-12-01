pub type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    pub const fn flip(self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
            Dir::East => Dir::West,
        }
    }
}

pub const fn deltas(x: usize, y: usize) -> [(Coord, Dir); 4] {
    [
        ((x, y.saturating_sub(1)), Dir::North),
        ((x, y + 1), Dir::South),
        ((x.saturating_sub(1), y), Dir::West),
        ((x + 1, y), Dir::East),
    ]
}

pub fn parse_with_lens<'a, V, F>(
    lines: &'a str,
    f: &'a F,
) -> (Coord, impl Iterator<Item = (Coord, V)> + 'a)
where
    F: Fn(u8) -> V,
{
    let y_len = lines.lines().count();
    let x_len = lines.lines().next().map(|s| s.trim().len()).unwrap();
    let it = lines.lines().enumerate().flat_map(move |(y, line)| {
        line.trim()
            .bytes()
            .enumerate()
            .map(move |(x, b)| ((x, y), f(b)))
    });
    ((x_len, y_len), it)
}

pub const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub const fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}
