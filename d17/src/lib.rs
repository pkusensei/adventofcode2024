use std::{mem, str::FromStr};

use itertools::Itertools;

pub fn p1(s: &str) -> String {
    let mut bot = Bot::from_str(s).unwrap();
    bot.eval(false).to_string().chars().join(",")
}

pub fn p2(s: &str) -> usize {
    let bot = Bot::from_str(s).unwrap();
    let target = bot.prog.iter().fold(0, |acc, n| acc * 10 + n);
    let mut cands = vec![0];
    let mut next = Vec::with_capacity(16);
    for digit in 0..=target.ilog10() {
        let seek = target % 10usize.pow(1 + digit);
        next.clear();
        for &initial in cands.iter() {
            let shifted = initial << 3;
            next.extend((0..8).map(|offset| shifted + offset).filter(|&a| {
                let mut bot = Bot {
                    ra: a,
                    ..bot.clone()
                };
                bot.eval(true) == seek
            }));
        }
        mem::swap(&mut cands, &mut next);
    }
    cands.into_iter().min().unwrap()
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
    out: usize,
    prog: Vec<usize>,
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

    fn eval(&mut self, p2: bool) -> usize {
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
                    } else if p2 {
                        return self.out;
                    }
                }
                // bxc
                4 => self.rb ^= self.rc,
                // out
                5 => self.out = self.out * 10 + self.combo(operand) % 8,
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
        self.out
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
