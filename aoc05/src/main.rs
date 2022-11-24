use std::io::{stdin, Read};

fn main() {
    let mut input = stdin();
    let mut cnt = String::new();
    if input.read_to_string(&mut cnt).is_ok() {
        println!("part1: {}", part1(&cnt));
        println!("part2: {}", part2(&cnt));
    }
}

fn part1(input: &str) -> usize {
    let mut polymer = input.as_bytes().to_vec();
    remove_react(&mut polymer).len()
}

fn part2(input: &str) -> usize {
    (b'a'..=b'z')
        .map(|b| {
            input
                .bytes()
                .filter(|&x| x != b && x != b - 32)
                .collect::<Vec<_>>()
        })
        .map(|mut polymer| remove_react(&mut polymer).len())
        .min()
        .unwrap_or(0)
}

fn remove_react(polymer: &mut Vec<u8>) -> &mut Vec<u8> {
    fn remove<'a, 'b>(polymer: &'a mut Vec<u8>, remains: &'b mut Vec<u8>) -> &'a mut Vec<u8> {
        let mut reacted = false;
        let mut i = 0;
        while !polymer.is_empty() && i < polymer.len() - 1 {
            if react(polymer[i], polymer[i + 1]) {
                reacted = true;
                i += 2;
                continue;
            }
            remains.push(polymer[i]);
            i += 1;
            if i == polymer.len() - 1 {
                remains.push(polymer[i]);
            }
        }

        std::mem::swap(polymer, remains);
        remains.clear();
        if reacted {
            remove(polymer, remains)
        } else {
            polymer
        }
    }

    remove(polymer, &mut vec![])
}

fn react(a: u8, b: u8) -> bool {
    if a > b {
        a - b == 32
    } else {
        b - a == 32
    }
}
