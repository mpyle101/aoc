
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

fn part_one(input: &str) -> i32 {
    input.split("\n")
        .fold(0, |acc, s| {
            let (c1, c2) = s.split_at(s.len() / 2);
            let c = find_common2(c1, c2);
            acc + (if c <= b'Z' { c - b'A' + 27 } else { c - b'a' + 1 } as i32)
        })
}

fn part_two(input: &str) -> i32 {
    let mut priorities = 0;
    let mut rucks = input.split('\n');
    while let Some(s1) = rucks.next() {
        let s2 = rucks.next().unwrap();
        let s3 = rucks.next().unwrap();
        let c  = find_common3(s1, s2, s3);
        priorities += (if c <= b'Z' { c - b'A' + 27 } else { c - b'a' + 1 } as i32);
    }
    
    priorities
}

fn find_common2(s1: &str, s2: &str) -> u8 {
    let b2 = s2.as_bytes();
    for b in s1.as_bytes() {
        if b2.contains(b) { return *b }
    }

    panic!("Common type not found!")
}

fn find_common3(s1: &str, s2: &str, s3: &str) -> u8 {
    let b2 = s2.as_bytes();
    let b3 = s3.as_bytes();
    for b in s1.as_bytes() {
        if b2.contains(b) && b3.contains(b) { return *b }
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