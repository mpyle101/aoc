fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input, 25, 25);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input, 25, 25);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str, preamble: usize, lookback: usize) -> u64
{
    let xmas = input.lines()
        .flat_map(|s| s.parse::<u64>())
        .collect::<Vec<_>>();

    step_one(&xmas, preamble, lookback)
}

fn part_two(input: &str, preamble: usize, lookback: usize) -> u64
{
    let xmas = input.lines()
        .flat_map(|s| s.parse::<u64>())
        .collect::<Vec<_>>();

    let n = step_one(&xmas, preamble, lookback);
    for i in 0..xmas.len()-1 {
        let mut j = i + 1;
        let mut sum = xmas[i];
        while sum < n { sum += xmas[j]; j += 1 }
        if sum == n {
            let mut seq = xmas[i..j-1].to_vec();
            seq.sort();
            return seq.first().unwrap() + seq.last().unwrap();
        }
    }

    0
}

fn step_one(xmas: &[u64], preamble: usize, lookback: usize) -> u64
{
    (preamble..xmas.len())
        .take_while(|&i| valid(xmas[i], &xmas[i-lookback..i]))
        .last()
        .map_or(0, |i| xmas[i+1])
}

fn valid(n: u64, xmas: &[u64]) -> bool
{
    for i in 0..xmas.len()-1 {
        for j in i..xmas.len() {
            if n == xmas[i] + xmas[j] {
                return true
            }
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input, 25, 25), 1124361034);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input, 25, 25), 129444555);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input, 5, 5), 127);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input, 5, 5), 62);
    }

}