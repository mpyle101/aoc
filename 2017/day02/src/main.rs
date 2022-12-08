fn main() {
    use std::{fs, time::Instant};

    let input = fs::read_to_string("./input.txt").unwrap();

    let t = Instant::now();
    let checksum = part_one(&input);
    println!("Part 1: {} {:?}", checksum, t.elapsed());

    let t = Instant::now();
    let checksum = part_two(&input);
    println!("Part 2: {} {:?}", checksum, t.elapsed());
}

fn part_one(input: &str) -> i32 {
    input.lines().map(|l| {
        let mut v: Vec<_> = l.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        v.sort_unstable();
        v.last().unwrap() - v.first().unwrap()
    })
    .sum()
}

fn part_two(input: &str) -> i32 {
    use itertools::Itertools;

    input.lines().map(|l| {
        let mut v: Vec<_> = l.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        v.sort_unstable();

        // After sorting, combinations will always have the higher
        // value as the second value in the pair.
        v.iter().combinations(2)
            .filter_map(|v| (v[1] % v[0] == 0).then_some(v[1] / v[0]))
            .sum::<i32>()
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();

        let checksum = part_one(&input);
        assert_eq!(checksum, 46402);

        let checksum = part_two(&input);
        assert_eq!(checksum, 265);
    }
}