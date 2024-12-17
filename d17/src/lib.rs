use std::str::FromStr;

use itertools::Itertools;

pub fn p1(s: &str) -> String {
    let mut bot = Bot::from_str(s).unwrap();
    bot.eval();
    bot.out.into_iter().join(",")
}

pub fn p2(s: &str) -> usize {
    let bot = Bot::from_str(s).unwrap();
    let n = bot.prog.len();
    let mut stack = vec![(0, 0)];
    let mut res = vec![];
    while let Some((a, idx)) = stack.pop() {
        if idx == bot.prog.len() {
            res.push(a);
            continue;
        }
        for b in 0..8 {
            let ra = (a << 3) | b;
            let seek = bot.prog[n - idx - 1];
            let mut curr = Bot { ra, ..bot.clone() };
            curr.eval();
            if curr.out[0] == seek {
                stack.push((ra, 1 + idx));
            }
        }
    }
    res.into_iter().min().unwrap()
}

fn parse(s: &str) -> impl Iterator<Item = usize> + '_ {
    s.split(|c: char| !c.is_ascii_digit())
        .filter_map(|v| v.parse().ok())
}

#[derive(Debug, Clone, Default)]
struct Bot {
    ra: usize,
    rb: usize,
    rc: usize,
    ip: usize,
    prog: Vec<usize>,
    out: Vec<usize>,
}

impl FromStr for Bot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = parse(s);
        let bot = Bot {
            ra: it.next().ok_or(())?,
            rb: it.next().ok_or(())?,
            rc: it.next().ok_or(())?,
            prog: it.collect(),
            ..Default::default()
        };
        Ok(bot)
    }
}

impl Bot {
    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            _ => unreachable!(),
        }
    }

    fn eval(&mut self) {
        while let (Some(&op), Some(&operand)) = (self.prog.get(self.ip), self.prog.get(1 + self.ip))
        {
            match op {
                // adv
                0 => {
                    let den = 2usize.pow(self.combo(operand) as u32);
                    self.ra /= den;
                }
                // bxl
                1 => self.rb ^= operand,
                // bst
                2 => self.rb = self.combo(operand) % 8,
                // jnz
                3 => {
                    if self.ra > 0 {
                        self.ip = operand;
                        continue; // skip +=1
                    }
                }
                // bxc
                4 => self.rb ^= self.rc,
                // out
                5 => self.out.push(self.combo(operand) % 8),
                // bdv
                6 => {
                    let den = 2usize.pow(self.combo(operand) as u32);
                    self.rb = self.ra / den;
                }
                // cdv
                7 => {
                    let den = 2usize.pow(self.combo(operand) as u32);
                    self.rc = self.ra / den;
                }
                _ => unreachable!(),
            }
            self.ip += 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(TEST), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(p1(INPUT), "7,0,7,3,4,1,3,0,1");

        assert_eq!(p2(TEST), 117440);
        assert_eq!(p2(INPUT), 156985331222018);
    }
}
