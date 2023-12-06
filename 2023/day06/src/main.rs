fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    let (line1, line2) = input.split_once('\n').unwrap();

    let (_, values) = line1.split_once(':').unwrap();
    let mut times = values.split_whitespace();
    let (_, values) = line2.split_once(':').unwrap();

    values.split_whitespace()
        .flat_map(|v| v.parse::<u32>())
        .map(|d| (times.next().map_or(0, |v| v.parse::<u32>().unwrap()), d))
        .map(|(t, d)|
            (1..t)
                .map(|n| n * (t - n))
                .filter(|&dist| dist > d)
                .count() as u32
        )
        .product()
}

fn part_two(input: &str) -> u64
{
    let (line1, line2) = input.split_once('\n').unwrap();

    let (_, s) = line1.split_once(':').unwrap();
    let v: String = s.split_whitespace().collect();
    let time: u64 = v.parse().unwrap();

    let (_, s) = line2.split_once(':').unwrap();
    let v: String = s.split_whitespace().collect();
    let dist: u64 = v.parse().unwrap();

    (1..time).map(|n| n * (time - n))
        .filter(|&d| d > dist)
        .count() as u64
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 345015);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 42588603);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 288);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 71503);
    }
}
