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

fn part_one(input: &str) -> u64
{
    input.lines().map(|line| extract(line, 2)).sum()
}

fn part_two(input: &str) -> u64
{
    input.lines().map(|line| extract(line, 12)).sum()
}

fn extract(line: &str, mut k: usize) -> u64
{
    let v = line.chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut n = 0;
    while k > 0 {
        n *= 10;

        // Find the next largest value up to the end less values left to fill
        let next = v[i..v.len() - k + 1]
            .iter()
            .enumerate()
            .fold((i, 0), |a, v| if *v.1 > a.1 { (v.0 + i, *v.1) } else { a });
        k -= 1;
        i = next.0 + 1;
        n += next.1;
    }

    n
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 17430);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 171975854269367);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 357);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 3121910778619);
    }
}
