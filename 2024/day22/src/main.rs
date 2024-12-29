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

fn part_one(input: &str) -> i64
{
    input.lines()
        .flat_map(|line| line.parse::<i64>())
        .map(|n| (0..2000).fold(n, |acc, _| evolve(acc)))
        .sum()
}

fn part_two(input: &str) -> i64
{
    use std::collections::{HashMap, HashSet};
    type Tracker = HashMap<i64, i64>;

    let prices = input.lines()
        .flat_map(|line| line.parse::<i64>())
        .map(|n| {
            let mut m = n;
            let mut v = (0..2000).map(|_| { m = evolve(m); m % 10 })
                .collect::<Vec<_>>();
            v.insert(0, n % 10);
            v
        })
        .collect::<Vec<_>>();
    let deltas = prices.iter()
        .map(|v| v.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut m = HashMap::new();
    for (d, p) in deltas.iter().zip(prices) {
        let mut seen = HashSet::new();
        for (w, i) in d.windows(4).zip(4..).filter(|(w, _)| seen.insert(*w)) {
            let n = p[i];
            *m.entry(w).or_insert(Tracker::new()).entry(n).or_default() += 1;
        }
    }
    
    m.values()
        .map(|m| m.iter().map(|(a, b)| a * b).sum())
        .max()
        .unwrap()
}

fn evolve(mut n: i64) -> i64
{
    let p = n * 64;
    n ^= p;
    n %= 16777216;

    let d = n / 32;
    n ^= d;
    n %= 16777216;

    let p = n * 2048;
    n ^= p;
    n %= 16777216;

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 13584398738);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1612);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 37327623);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example2.txt");
        assert_eq!(part_two(input), 23);
    }
}
