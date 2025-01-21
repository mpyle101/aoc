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

fn part_one(input: &str) -> i64
{
    use itertools::Itertools;

    let weights = load(input);
    let target  = weights.iter().sum::<i64>() / 3;

    // Manually worked our way to 6 because 2-5 returned no results.
    // Could put an outer loop to work our way up to handle any data
    // set.
    weights.iter()
        .combinations(6)
        .filter_map(|v| (v.iter().copied().sum::<i64>() == target).then_some(v))
        .map(|v| v.iter().copied().product::<i64>())
        .min()
        .unwrap()
}

fn part_two(input: &str) -> i64
{
    use itertools::Itertools;

    let weights = load(input);
    let target  = weights.iter().sum::<i64>() / 4;

    // Manually worked our way to 4 because 2 & 3 returned no results.
    // Could put an outer loop to work our way up to handle any data
    // set.
    weights.iter()
        .combinations(4)
        .filter_map(|v| (v.iter().copied().sum::<i64>() == target).then_some(v))
        .map(|v| v.iter().copied().product::<i64>())
        .min()
        .unwrap()
}

fn load(input: &str) -> Vec<i64>
{
    input.lines()
        .flat_map(|line| line.parse::<i64>())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 11846773891);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 80393059);
    }
}