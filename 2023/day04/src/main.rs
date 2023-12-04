fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let cards = part_one::<10>(input);
    println!("Part 1: {} ({:?})", cards, t.elapsed());

    let t = Instant::now();
    let cards = part_two::<10>(input);
    println!("Part 2: {} ({:?})", cards, t.elapsed());
}

fn part_one<const N: usize>(input: &str) -> u32
{
    let mut winners = [0; N];

    input.lines()
        .map(|line| {
            let (_, c) = line.split_once(':').unwrap();
            let (s1, s2) = c.split_once('|').unwrap();
            s1.split_whitespace()
                .enumerate()
                .flat_map(|(i, s)| s.parse::<u32>().map(|n| (i, n)))
                .for_each(|(i, n)| { winners[i] = n; });
            let count = s2.split_whitespace()
                .flat_map(|v| v.parse::<u32>())
                .filter(|n| winners.contains(n))
                .count();

            if count == 0 { 
                0
            } else {
                2u32.pow(count as u32 - 1)
            }
        })
        .sum()
}

fn part_two<const N: usize>(input: &str) -> u32
{
    let mut winners = [0; N];

    let counts: Vec<usize> = input.lines()
        .map(|line| {
            let (_, c) = line.split_once(':').unwrap();
            let (s1, s2) = c.split_once('|').unwrap();
            s1.split_whitespace()
                .enumerate()
                .flat_map(|(i, s)| s.parse::<u32>().map(|n| (i, n)))
                .for_each(|(i, n)| { winners[i] = n; });
            s2.split_whitespace()
                .flat_map(|v| v.parse::<u32>())
                .filter(|n| winners.contains(n))
                .count()
        })
        .collect();

    let mut cards = vec![1; counts.len()];
    for  idx in 0..cards.len() {
        let count = counts[idx];
        (idx+1..=idx+count).for_each(|i| cards[i] += cards[idx]);
    }

    cards.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one::<10>(input), 32609);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two::<10>(input), 14624680);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one::<5>(input), 13);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two::<5>(input), 30);
    }
}
