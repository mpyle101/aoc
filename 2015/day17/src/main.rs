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
    use itertools::Itertools;

    let containers = load(input);
    (4..containers.len())
        .flat_map(|n| containers.iter()
            .combinations(n)
            .filter(|v| v.iter().cloned().sum::<i32>() == 150)
        )
        .count()
}

fn part_two(input: &str) -> usize
{
    use itertools::Itertools;

    let containers = load(input);
    let mut seqs = (4..containers.len())
        .flat_map(|n| containers.iter()
            .combinations(n)
            .filter(|v| v.iter().cloned().sum::<i32>() == 150)
            .map(|v| v.len())
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    seqs.sort_unstable();

    seqs.iter()
        .filter(|&n| *n == seqs[0])
        .count()
}

fn load(input: &str) -> Vec<i32>
{
    input.lines()
        .flat_map(|s| s.parse::<i32>())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 4372);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 4);
    }
}