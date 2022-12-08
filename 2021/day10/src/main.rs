
fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines = load(&input);

    let t1 = Instant::now();
    let (score, inc) = part_one(&lines);
    let t2 = Instant::now();
    println!("Part 1: {} {:?}", score, t2 - t1);

    let t1 = Instant::now();
    let score = part_two(&inc);
    let t2 = Instant::now();
    println!("Part 2: {} {:?}", score, t2 - t1);
}

fn load(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part_one<'a>(lines: &[&'a str]) -> (i32, Vec<&'a str>) {
    use std::collections::HashMap;

    let mut inc  = Vec::new();
    let points   = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let brackets = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    
    let score = lines.iter().map(|&l| {
        let mut stack = Vec::new();
        for c in l.chars() {
            if let Some(b) = brackets.get(&c) {
                if let Some(v) = stack.pop() {
                    if v != *b {
                        return *points.get(&c).unwrap();
                    }
                }
            } else {
                stack.push(c);
            }
        };
        if stack.len() > 0 { inc.push(l) }
        0
    })
    .sum();

    (score, inc)
}

fn part_two(lines: &[&str]) -> i64 {
    use std::collections::{HashMap, HashSet};

    let points  = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let closers = HashSet::from([')',']', '}', '>']);
    let mut scores = lines.iter().map(|l| {
        let stack = l.chars().fold(Vec::new(), |mut v, c| {
            if closers.contains(&c) { v.pop(); } else { v.push(c); };
            v
        });
        stack.iter().rev().fold(0, |acc, c| acc * 5 + points.get(c).unwrap())
    })
    .collect::<Vec<i64>>();

    scores.sort();
    scores[scores.len() / 2]
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let lines = load(&input);

        let (score, inc) = part_one(&lines);
        assert_eq!(score, 413733);

        let score = part_two(&inc);
        assert_eq!(score, 3354640192);
    }
}