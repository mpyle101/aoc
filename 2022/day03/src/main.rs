
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
            let c = match iter.next() {
                Some(b) => 
                    if [$($arr),+].iter().all(|s| s.contains(b)) { 
                        b as u8 
                    } else { 
                        continue
                    }, 
                None => panic!("Common type not found!")
            };

            break (if c <= b'Z' { c - b'A' + 27 } else { c - b'a' + 1 }) as i32
        }
    }};
}

fn part_one(input: &str) -> i32 {
    input.split("\n")
        .fold(0, |acc, s| {
            let (s1, s2) = s.split_at(s.len() / 2);
            acc + priority!(s1, s2)
        })
}

fn part_two(input: &str) -> i32 {
    let mut priorities = 0;
    let mut rucks = input.split('\n');
    while let Some(s1) = rucks.next() {
        let s2 = rucks.next().unwrap();
        let s3 = rucks.next().unwrap();
        priorities += priority!(s1, s2, s3);
    }
    
    priorities
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