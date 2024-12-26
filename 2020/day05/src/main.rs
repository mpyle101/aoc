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
    let mut seats = input.lines()
        .map(seat)
        .collect::<Vec<_>>();
    seats.sort_unstable();

    // take_while runs until the test fails so the last element
    // will be the last value for which the predicate returned
    // true. We know it's a full flight so the last true test
    // must be the seat before the open seat.
    // Ex: 10 11 12 14
    // 11 => 11 - 10 == 1  (true)
    // 12 => 12 - 11 == 1  (true)
    // 14 => 14 - 12 == 2  (false)
    // The last true is for 12 - 11, so the iterations last
    // value is 12 and the missing one is 13
    seats.iter()
        .enumerate()
        .skip(1)
        .take_while(|(i, n)| **n - seats[i-1] == 1)
        .last()
        .map_or(0, |(_, n)| n + 1)
}

fn seat(bp: &str) -> u32
{
    bp.chars()
        .rev()
        .zip(0..)
        .filter(|&(c, _)| c == 'B' || c == 'R')
        .fold(0, |n, (_, i)| n | 1 << i)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 998);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 676);
    }
}