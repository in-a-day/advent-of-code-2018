use std::{collections::HashMap, io::Read, str::FromStr};

use regex::Regex;

#[macro_use]
extern crate lazy_static;

type Point = (usize, usize);

fn main() {
    let mut input = std::io::stdin();
    let mut content = String::new();
    input
        .read_to_string(&mut content)
        .expect("read input error");

    let mut claims = vec![];
    let mut fabric = HashMap::new();
    for line in content.lines() {
        let claim = line.parse::<Claim>().unwrap();
        for p in claim.iter_points() {
            *fabric.entry(p).or_default() += 1;
        }
        claims.push(claim);
    }

    println!("{}", part1(&fabric));
    println!("{}", part2(&fabric, &claims));
}

fn part1(fabric: &HashMap<Point, usize>) -> usize {
    fabric.values().filter(|&&v| v > 1).count()
}

fn part2(fabric: &HashMap<Point, usize>, claims: &Vec<Claim>) -> usize {
    for claim in claims {
        if claim
            .iter_points()
            .all(|ref p| *fabric.get(p).unwrap() == 1)
        {
            return claim.id;
        }
    }

    unreachable!()
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    row_start: usize,
    row_len: usize,
    col_start: usize,
    col_len: usize,
}

impl Claim {
    fn iter_points(&self) -> ClaimPoints {
        ClaimPoints::new(self)
    }
}

#[derive(Debug)]
struct ClaimPoints<'a> {
    claim: &'a Claim,
    x: usize,
    y: usize,
}

impl<'a> ClaimPoints<'a> {
    fn new(claim: &'a Claim) -> Self {
        ClaimPoints { claim, x: 0, y: 0 }
    }
}

impl<'a> Iterator for ClaimPoints<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.claim.row_len {
            return None;
        }
        let p = (self.claim.row_start + self.x, self.claim.col_start + self.y);
        if self.y == self.claim.col_len - 1 {
            self.y = 0;
            self.x += 1;
        } else {
            self.y += 1;
        }

        Some(p)
    }
}

impl FromStr for Claim {
    type Err = String;

    // TODO 使用RE解析字符串为Claim
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                   \#
                   (?P<id>\d+)
                   \s+@\s+
                   (?P<col_start>\d+),(?P<row_start>\d+):
                   \s+
                   (?P<col_len>\d+)x(?P<row_len>\d+)"
            )
            .unwrap();
        }
        let caps = RE.captures(s).unwrap();

        Ok(Claim {
            id: caps["id"].parse().unwrap(),
            row_start: caps["row_start"].parse().unwrap(),
            row_len: caps["row_len"].parse().unwrap(),
            col_start: caps["col_start"].parse().unwrap(),
            col_len: caps["col_len"].parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_test() {
        let x = "#1 @ 393,863: 11x29";
        let x: Vec<_> = x.split_whitespace().collect();
        println!("{:?}", x);
    }

    #[test]
    fn claim_points_test() {
        let cp = ClaimPoints::new(&Claim {
            id: 1,
            row_start: 1,
            row_len: 3,
            col_start: 1,
            col_len: 3,
        });
        let v: Vec<_> = cp.into_iter().collect();
        assert_eq!(v.len(), 9);
    }

    #[test]
    fn claim_from_str_test() {
        let s = "#1 @ 393,863: 11x29";
        let c = s.parse::<Claim>().unwrap();
        assert_eq!(
            c,
            Claim {
                id: 1,
                col_start: 393,
                col_len: 11,
                row_start: 863,
                row_len: 29,
            }
        );
    }
}
