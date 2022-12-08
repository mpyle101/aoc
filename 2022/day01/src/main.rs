
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
    input.split("\n\n")
        .map(|group| group.split('\n')
            .map(|s| s.parse::<i32>().unwrap())
            .sum())
        .max()
        .unwrap()
}

fn part_two(input: &str) -> i32 {
    use std::collections::BinaryHeap;

    let mut calories = input.split("\n\n")
        .map(|group| group.split('\n')
            .map(|s| s.parse::<i32>().unwrap())
            .sum())
        .collect::<BinaryHeap<i32>>();

    return (0..3).map(|_| calories.pop().unwrap()).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let calories = part_one(&input);
        assert_eq!(calories, 70720);

        let calories = part_two(&input);
        assert_eq!(calories, 207148);
    }
}