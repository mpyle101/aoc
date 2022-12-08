fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let calories = part_one(input);
    println!("Part 1: {} ({:?})", calories, t.elapsed());

    let t = Instant::now();
    let calories = part_two(input);
    println!("Part 2: {} ({:?})", calories, t.elapsed());
}

macro_rules! priority {
    ($s1:expr, $($arr:expr),+) => {{
        let mut iter = $s1.chars();
        loop {
            let b = iter.next().unwrap();
            if [$($arr),+].iter().all(|s| s.contains(b)) {
                let c = b as u8;
                break (if c <= b'Z' { c - b'A' + 27 } else { c - b'a' + 1 }) as i32
            }
        }
    }};
}

fn part_one(input: &str) -> i32 {
    input.lines().fold(0, |acc, s| {
        let (s1, s2) = s.split_at(s.len() / 2);
        acc + priority(s1, |c| s2.contains(c).then_some(c as u8))
    })
}

fn part_two(input: &str) -> i32 {
    let mut priorities = 0;
    let mut rucks = input.lines();
    while let Some(s1) = rucks.next() {
        let (s2, s3) = (rucks.next().unwrap(), rucks.next().unwrap());
        priorities += priority!(s1, s2, s3);
    }

    priorities
}

fn priority(s: &str, f: impl Fn(char) -> Option<u8>) -> i32 {
    s.chars()
        .find_map(f)
        .map_or(0, |c| (if c <= b'Z' { c - b'A' + 27 } else { c - b'a' + 1 }) as i32)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let priorities = part_one(input);
        assert_eq!(priorities, 7793);

        let priorities = part_two(input);
        assert_eq!(priorities, 2499);
    }
}
