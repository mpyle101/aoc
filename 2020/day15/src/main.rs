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
    let v = input.split(',')
      .flat_map(|s| s.parse::<u32>())
      .collect::<Vec<_>>();

    play(&v, 2020)
}

fn part_two(input: &str) -> u32
{
    let v = input.split(',')
        .flat_map(|s| s.parse::<u32>())
        .collect::<Vec<_>>();

    play(&v, 30000000)
}

fn play(nums: &[u32], iterations: u32) -> u32
{
    use std::collections::HashMap;

    let mut spoken: HashMap<_,_> = nums.iter()
        .zip(0..)
        .map(|(n, i)| (*n, (i+1, i+1)))
        .collect();

    let last  = nums[nums.len() - 1];
    let turn = (spoken.len() + 1) as u32;
    (turn..=iterations)
        .fold(last, |last, i| {
            let (a, b) = spoken.entry(last).or_insert((i, i));
            let n = *b - *a;
            let e = spoken.entry(n).or_insert((i, i));
            *e = (e.1, i);
            n
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 211);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 2159626);
    }

    #[test]
    fn example_part_one()
    {
        assert_eq!(part_one("3,1,2"), 1836);
    }

    #[test]
    fn example_part_two()
    {
        assert_eq!(part_two("2,3,1"), 6895259);
    }
}