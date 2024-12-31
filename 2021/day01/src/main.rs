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
    let v = input.lines()
        .flat_map(|s| s.parse::<u32>())
        .collect::<Vec<_>>();

    v.windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}

fn part_two(input: &str) -> u32
{
    let v = input.lines()
        .flat_map(|s| s.parse::<u32>())
        .collect::<Vec<_>>();

    let mut count = 0;
    v.windows(3)
        .fold(u32::MAX, |last, w| {
            let n = w.iter().sum();
            count += (n > last) as u32;
            n
        });

    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1676);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1706);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 7);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 5);
    }

}