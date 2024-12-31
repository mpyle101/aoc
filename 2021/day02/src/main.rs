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
    input.lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(d, s)| (d, s.parse::<u32>().unwrap()))
        .fold([0, 0], |[x, y], (d, n)| 
            match d {
                "up"      => [x, y - n],
                "down"    => [x, y + n],
                "forward" => [x + n, y],
                _ => unreachable!()
            }
        )
        .iter()
        .product()
}

fn part_two(input: &str) -> u64
{
    input.lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(d, s)| (d, s.parse::<u64>().unwrap()))
        .fold([0, 0, 0], |[a, x, y], (d, n)| {
            match d {
                "up"      => [a - n, x, y],
                "down"    => [a + n, x, y],
                "forward" => [a, x + n, y + n * a],
                _ => unreachable!()
            }
        })
        .iter()
        .skip(1)
        .product()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1924923);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1982495697);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 150);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 900);
    }

}