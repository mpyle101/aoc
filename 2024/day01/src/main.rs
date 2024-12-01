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

fn part_one(input: &str) -> u32
{
    let (mut v1, mut v2) = input.lines()
        .fold((Vec::new(), Vec::new()), |mut acc, line| {
            let mut it = line.split_whitespace();
            acc.0.push(it.next().unwrap().parse::<u32>().unwrap());
            acc.1.push(it.next().unwrap().parse::<u32>().unwrap());
            acc
        });

    v1.sort_unstable();
    v2.sort_unstable();
    v1.iter()
        .zip(v2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part_two(input: &str) -> u32
{
    use std::collections::HashMap;

    let (m1, m2) = input.lines()
        .fold((HashMap::new(), HashMap::new()), |mut acc, line| {
            let mut it = line.split_whitespace();
            let v = it.next().unwrap().parse::<u32>().unwrap();
            *acc.0.entry(v).or_insert(0) += 1;
            let v = it.next().unwrap().parse::<u32>().unwrap();
            *acc.1.entry(v).or_insert(0) += 1;
            acc
        });

    m1.iter()
        .map(|(k, v)| m2.get(k).map_or(0, |n| k * v * n))
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 3714264);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 18805872);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 11);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 31);
    }
}
