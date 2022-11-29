use std::collections::HashSet;
use std::io::{stdin, Read};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = stdin();
    let mut cnt = String::new();
    input.read_to_string(&mut cnt)?;

    if let Ok(res) = part1(&cnt) {
        println!("{res}");
    }

    Ok(())
}

fn part1(input: &str) -> Result<String> {
    let mut records = vec![];
    for line in input.lines() {
        let record: Record = line.parse()?;
        records.push(record);
    }

    Ok(Steps::from(records).iter().collect())
}

fn to_upper(idx: usize) -> char {
    (idx + 65) as u8 as char
}

fn to_index(c: char) -> usize {
    c as usize - 65
}

pub struct Steps {
    steps: Vec<Option<HashSet<char>>>,
}

impl Steps {
    fn iter(&mut self) -> StepIter {
        StepIter::new(&mut self.steps)
    }
}

impl From<Vec<Record>> for Steps {
    fn from(records: Vec<Record>) -> Self {
        let mut steps: Vec<Option<HashSet<char>>> = (0..26).map(|_| None).collect();
        for record in records {
            steps[to_index(record.dependency)].get_or_insert(HashSet::new());
            steps[to_index(record.step)]
                .get_or_insert(HashSet::new())
                .insert(record.dependency);
        }

        Self { steps }
    }
}

pub struct Record {
    step: char,
    dependency: char,
}

impl FromStr for Record {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(' ').collect();

        let dependency = words[1].as_bytes()[0] as char;
        let step = words[7].as_bytes()[0] as char;

        Ok(Self { step, dependency })
    }
}

pub struct StepIter<'a> {
    // index is step_char - 65, value is dependency
    // len is 26, None represents did not have this char
    // empty hashset represents step_char did not have dependency
    steps: &'a mut Vec<Option<HashSet<char>>>,
}

impl<'a> StepIter<'a> {
    fn new(steps: &'a mut Vec<Option<HashSet<char>>>) -> Self {
        Self { steps }
    }
}

impl<'a> Iterator for StepIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let next_step = self
            .steps
            .iter_mut()
            .enumerate()
            .find(|(_, val)| val.as_ref().map_or(false, |v| v.is_empty()));

        let (step, depends) = next_step?;
        *depends = None;
        let step = to_upper(step);
        for ele in self.steps.iter_mut() {
            if ele.is_some() {
                ele.as_mut().unwrap().remove(&step);
            }
        }

        Some(step)
    }
}
