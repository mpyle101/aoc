use std::collections::HashSet;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    input.split("\n\n")
        .map(|group| {
            group.lines()
                .map(|line| HashSet::from_iter(line.chars()))
                .fold(HashSet::new(), |set, group| &set | &group)
                .len()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    input.split("\n\n")
        .map(|group| {
            group.lines()
                .map(|line| HashSet::from_iter(line.chars()))
                .reduce(|set: HashSet<char>, group| &set & &group)
                .map_or(0, |set| set.len())
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 6430);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 3125);
    }
}