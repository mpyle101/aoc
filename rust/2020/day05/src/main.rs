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
    input.lines().map(seat).max().unwrap()
}

fn part_two(input: &str) -> u32
{

    0
}

fn seat(bp: &str) -> u32
{
    bp.chars().rev()
        .zip(0..)
        .fold(0, |n, (c, i)| {
            n | if c == 'F' || c == 'L' { 1 << i } else { n }
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = load(include_str!("../input.txt"));
        assert_eq!(part_one(&input), 998);
    }

    #[test]
    fn input_part_two()
    {
        let input = load(include_str!("../input.txt"));
        assert_eq!(part_two(&input), 676);
    }
}