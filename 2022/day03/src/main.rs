
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
    use std::collections::HashSet;

    input.split("\n")
        .map(|s| {
            let (c1, c2) = s.split_at(s.len() / 2);
            let s1 = c1.bytes().collect::<HashSet<_>>();
            let s2 = c2.bytes().collect::<HashSet<_>>();
            let c = s1.intersection(&s2).take(1).next().unwrap();
            (if *c <= b'Z' { c - b'A' + 27 } else { c - b'a' + 1 } as i32)
        })
        .sum::<i32>()
}

fn part_two(input: &str) -> i32 {
    use std::collections::HashSet;

    let mut priorities = 0;
    let mut rucks = input.split('\n');
    while let Some(ruck) = rucks.next() {
        let s1 = ruck.bytes().collect::<HashSet<_>>();
        let s2 = rucks.next().unwrap().bytes().collect::<HashSet<_>>();
        let s3 = rucks.next().unwrap().bytes().collect::<HashSet<_>>();
        let s4 = s1.intersection(&s2).cloned().collect::<HashSet<_>>();
        let c = s4.intersection(&s3).take(1).next().unwrap();
        priorities += (if *c <= b'Z' { c - b'A' + 27 } else { c - b'a' + 1 } as i32);
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