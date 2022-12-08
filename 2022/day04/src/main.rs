
fn main() {
    use std::fs;
    use std::time::Instant;

    let input = fs::read_to_string("./input.txt").unwrap();

    let t = Instant::now();
    let overlapping = part_one(&input);
    println!("Part 1: {} ({:?})", overlapping, t.elapsed());

    let t = Instant::now();
    let overlapping = part_two(&input);
    println!("Part 2: {} ({:?})", overlapping, t.elapsed());
}

fn part_one(input: &str) -> usize {
    input.split('\n')
        .filter(|s| {
            let mut iter = s.split(',');
            let (s1, s2) = (iter.next().unwrap(), iter.next().unwrap());
            let (r1, r2) = (get_range(s1), get_range(s2));

            (r1.0 <= r2.0 && r1.1 >= r2.1) || (r2.0 <= r1.0 && r2.1 >= r1.1)
        })
        .count()
}

fn part_two(input: &str) -> usize {
    input.split('\n')
        .filter(|s| {
            let mut iter = s.split(',');
            let (s1, s2) = (iter.next().unwrap(), iter.next().unwrap());
            let (r1, r2) = (get_range(s1), get_range(s2));

            r2.0 <= r1.1 && r2.1 >= r1.0
        })
        .count()
}

fn get_range(s: &str) -> (i32, i32) {
    let mut iter = s.split('-');
    (
        iter.next().unwrap().parse::<i32>().unwrap(),
        iter.next().unwrap().parse::<i32>().unwrap()
    )
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