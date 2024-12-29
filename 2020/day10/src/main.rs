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
    // The jolt change can only go up so the adapters must
    // go in sorted order which, for the problem to be solvable,
    // must represent the adapter order. So, we just need to
    // count the jumps of 1 and 3.
    let mut v = input.lines()
        .flat_map(|line| line.parse::<u32>())
        .collect::<Vec<_>>();
    v.sort_unstable();
    let last = *v.last().unwrap();
    v.insert(0, 0);
    v.push(last + 3);

    v.windows(2)
        .fold([0, 0], |arr, w| {
            [
                arr[0] + (w[1] - w[0] == 1) as u32,
                arr[1] + (w[1] - w[0] == 3) as u32,
            ]
        })
        .iter()
        .product()
}

fn part_two(input: &str) -> u64
{
    use std::cmp::min;

    let mut a = input.lines()
        .flat_map(|line| line.parse::<u32>())
        .collect::<Vec<_>>();
    a.sort_unstable();
    let last = *a.last().unwrap();
    a.insert(0, 0);
    a.push(last + 3);

    // The number of ways you can get to a value is the sum of
    // the ways to get to values which can reach that value.
    // Start with 1 way to get to the starting value and run
    // the length of the value array increasing the counts for
    // each value that can be reached from the current one.
    let mut dp = vec![0; a.len()];
    dp[0] = 1;
    (0..dp.len()).for_each(|i| {
        let end = min(i+3, dp.len()-1);
        (i+1..=end)
            .filter(|j| a[*j] - a[i] <= 3)
            .for_each(|j| dp[j] += dp[i]);
    });
    
    dp[dp.len() - 1]
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 2574);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 2644613988352);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 35);

        let input = include_str!("../example2.txt");
        assert_eq!(part_one(input), 220);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_two(input), 8);

        let input = include_str!("../example2.txt");
        assert_eq!(part_two(input), 19208);
    }
}