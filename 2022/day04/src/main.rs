
fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let overlapping = part_one(input);
    println!("Part 1: {} ({:?})", overlapping, t.elapsed());

    let t = Instant::now();
    let overlapping = part_two(input);
    println!("Part 2: {} ({:?})", overlapping, t.elapsed());
}

fn part_one(input: &str) -> usize {
    let check = |r1: &(i32, i32), r2: &(i32, i32)| r1.0 <= r2.0 && r1.1 >= r2.1;

    input.lines()
        .filter_map(|s| s.split_once(','))
        .map(|(s1, s2)| (range(s1), range(s2)))
        .filter(|(r1, r2)| check(r1, r2) || check(r2, r1))
        .count()
}

fn part_two(input: &str) -> usize {
    input.lines()
        .filter_map(|s| s.split_once(','))
        .map(|(s1, s2)| (range(s1), range(s2)))
        .filter(|(r1, r2)| r2.0 <= r1.1 && r2.1 >= r1.0)
        .count()
}

fn range(s: &str) -> (i32, i32) {
    s.split_once('-')
        .map(|(r1, r2)| (
            r1.parse::<i32>().unwrap(),
            r2.parse::<i32>().unwrap()
        ))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let overlapping = part_one(input);
        assert_eq!(overlapping, 494);

        let overlapping = part_two(input);
        assert_eq!(overlapping, 833);
    }
}