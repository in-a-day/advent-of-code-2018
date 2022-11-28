use std::{
    collections::{HashMap, HashSet},
    io::{stdin, Read},
};

fn main() {
    let mut input = stdin();
    let mut cnt = String::new();
    input.read_to_string(&mut cnt).unwrap();

    println!("Hello, world!");
}

fn part1(input: &str) {
    let mut steps: Vec<Option<HashSet<&str>>> = (0..26).map(|_| None).collect();

    let mut step2depends = HashMap::new();
    for line in input.lines() {
        let mut x: Vec<&str> = input.split(" ").collect();
        if x.len() != 10 {
            panic!("parse err");
        }
        let step = x[1];
        let step_before = x[7];

        step2depends.entry(step_before).or_insert(vec![]).push(step);
    }
}
