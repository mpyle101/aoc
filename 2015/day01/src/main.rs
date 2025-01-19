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

fn part_one(input: &str) -> i32
{
    input.chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .sum()
}

fn part_two(input: &str) -> usize
{
    input.chars()
        .zip(1..)
        .scan(0, |st, (c, i)| {
            *st += if c == '(' { 1 } else { -1 };
            if *st == -1 { None } else { Some(i) }
        })
        .last()
        .unwrap() + 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 280);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1797);
    }
}