use std::{
    collections::HashSet,
    io::{self, Read},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    let res: i32 = input
        .lines()
        .map(|v| v.parse::<i32>().expect("invalid input number"))
        .sum();
    Ok(res)
}

fn part2(input: &str) -> Result<i32> {
    let mut presented = HashSet::new();
    let mut curr = 0;
    presented.insert(curr);
    loop {
        for s in input.lines() {
            curr += s.parse::<i32>()?;
            if presented.contains(&curr) {
                return Ok(curr);
            }
            presented.insert(curr);
        }
    }
}
