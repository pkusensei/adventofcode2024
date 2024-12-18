use std::{collections::VecDeque, str::FromStr};

use itertools::Itertools;

pub fn p1(s: &str) -> String {
    let mut bot = Bot::from_str(s).unwrap();
    bot.eval().into_iter().join(",")
}

pub fn p2(s: &str) -> usize {
    let bot = Bot::from_str(s).unwrap();
    let mut queue = VecDeque::from([(0, bot.prog.as_slice())]);
    while let Some((a, prog)) = queue.pop_front() {
        if prog.is_empty() {
            return a;
        }
        let target = *prog.last().unwrap();
        for i in 0..8 {
            let mut curr = Bot {
                ra: i + 8 * a,
                ..bot.clone()
            };
            let out = curr.eval();
            if out.first().is_some_and(|&v| v == target) {
                let next = &prog[..prog.len() - 1];
                queue.push_back((i + 8 * a, next));
            }
        }
    }
    0
}

#[derive(Debug, Clone, Default)]
struct Bot {
    ra: usize,
    rb: usize,
    rc: usize,
    ip: usize,
    prog: Vec<usize>,
}

impl FromStr for Bot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s
            .split(|c: char| !c.is_ascii_digit())
            .filter_map(|v| v.parse().ok());
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

    fn eval(&mut self) -> Vec<usize> {
        let mut out = vec![];
        while let Some(&op) = self.prog.get(self.ip) {
            let operand = *self.prog.get(1 + self.ip).unwrap_or(&0);
            match op {
                // adv
                0 => self.ra >>= self.combo(operand),
                // bxl
                1 => self.rb ^= operand,
                // bst
                2 => self.rb = self.combo(operand) % 8,
                // jnz
                3 => {
                    if self.ra > 0 {
                        self.ip = operand;
                        continue; // skip +=2
                    }
                }
                // bxc
                4 => self.rb ^= self.rc,
                // out
                5 => out.push(self.combo(operand) % 8),
                // bdv
                6 => self.rb = self.ra >> self.combo(operand),
                // cdv
                7 => self.rc = self.ra >> self.combo(operand),
                _ => unreachable!(),
            }
            self.ip += 2;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    const TEST2: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn it_works() {
        assert_eq!(p1(TEST), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(p1(INPUT), "7,0,7,3,4,1,3,0,1");

        assert_eq!(p2(TEST2), 117440);
        assert_eq!(p2(INPUT), 156985331222018);
    }
}
