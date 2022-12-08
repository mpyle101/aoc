
fn main() {
    use std::fs;
    use std::time::Instant;

    let input = fs::read_to_string("./input.txt").unwrap();

    let t = Instant::now();
    let calories = part_one(&input);
    println!("Part 1: {} ({:?})", calories, t.elapsed());

    let t = Instant::now();
    let calories = part_two(&input);
    println!("Part 2: {} ({:?})", calories, t.elapsed());
}

macro_rules! priority {
    ($s1:expr, $($arr:expr),+) => {{
        let mut iter = $s1.chars();
        loop {
            match iter.next() {
                Some(b) => 
                    if [$($arr),+].iter().all(|s| s.contains(b)) { 
                        let c = b as u8;
                        break (if c <= b'Z' { c - b'A' + 27 } else { c - b'a' + 1 }) as i32
                    }, 
                None => panic!("Common type not found!")
            }
        }
    }};
}

fn part_one(input: &str) -> i32 {
    input.split("\n")
        .fold(0, |acc, s| {
            let (s1, s2) = s.split_at(s.len() / 2);
            acc + priority(s1, |c| s2.contains(c))
        })
}

fn part_two(input: &str) -> i32 {
    let mut priorities = 0;
    let mut rucks = input.split('\n');
    while let Some(s1) = rucks.next() {
        let s2 = rucks.next().unwrap();
        let s3 = rucks.next().unwrap();
        priorities += priority!(s1, s2, s3);
        // OR: priorities += priority(s1, |c| s2.contains(c) && s3.contains(c))
    }
    
    priorities
}

fn priority(s: &str, f: impl Fn(char) -> bool) -> i32 {
    for c in s.chars() {
        if f(c) {
            let b = c as u8;
            return (if b <= b'Z' { b - b'A' + 27 } else { b - b'a' + 1 }) as i32
        }
    }

    panic!("Common type not found!")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let priorities = part_one(&input);
        assert_eq!(priorities, 7793);

        let priorities = part_two(&input);
        assert_eq!(priorities, 2499);
    }
}