use std::{
    io::{self, Read},
    usize,
};

fn main() {
    let mut input = io::stdin();
    let mut content = String::new();
    input
        .read_to_string(&mut content)
        .expect("read input error");

    println!("{}", part1(&content));
    if let Some(p) = part2(&content) {
        println!("{p}");
    }
}

fn part1(input: &str) -> usize {
    let mut fre = [0u8; 26];
    let (mut two, mut three) = (0, 0);

    for line in input.lines() {
        for f in fre.iter_mut() {
            *f = 0;
        }
        for b in line.as_bytes().iter().map(|&b| b as usize - 97) {
            fre[b] = fre[b].saturating_add(1);
        }
        if fre.iter().any(|&v| v == 2) {
            two += 1;
        }
        if fre.iter().any(|&v| v == 3) {
            three += 1;
        }
    }

    two * three
}

fn part2(input: &str) -> Option<String> {
    let lines: Vec<_> = input.lines().collect();
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if let Some(p) = diff_string(lines[i], lines[j]) {
                return Some(p);
            }
        }
    }

    None
}

fn diff_string(str1: &str, str2: &str) -> Option<String> {
    if str1.len() != str2.len() {
        return None;
    }

    let mut has_one_diff = false;
    for (c1, c2) in str1.chars().zip(str2.chars()) {
        if c1 != c2 {
            if has_one_diff {
                return None;
            }
            has_one_diff = true;
        }
    }

    Some(
        str1.chars()
            .zip(str2.chars())
            .filter(|&(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect()
    )
}
