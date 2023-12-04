fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let cards = part_one(input);
    println!("Part 1: {} ({:?})", cards, t.elapsed());

    let t = Instant::now();
    let cards = part_two(input);
    println!("Part 2: {} ({:?})", cards, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    use std::collections::HashSet;

    input.lines()
        .map(|line| {
            let s = &line[9..];
            let (s1, s2) = s.split_once('|').unwrap();
            let winners: HashSet<u32> = s1.split(' ')
                .flat_map(|v| v.parse::<u32>())
                .collect();
            let mine: HashSet<u32> = s2.split(' ')
                .flat_map(|v| v.parse::<u32>())
                .collect();
            let good = &winners & &mine;
            if good.is_empty() { 
                0
            } else {
                2u32.pow(good.len() as u32 - 1)
            }
        })
        .sum()
}

fn part_two(input: &str) -> u32
{
    use std::collections::HashSet;

    let winners: Vec<usize> = input.lines()
        .map(|line| {
            let s = &line[9..];
            let (s1, s2) = s.split_once('|').unwrap();
            let winners: HashSet<u32> = s1.split(' ')
                .flat_map(|v| v.parse::<u32>())
                .collect();
            let mine: HashSet<u32> = s2.split(' ')
                .flat_map(|v| v.parse::<u32>())
                .collect();
            let good = &winners & &mine;
            good.len()
        })
        .collect();

    let mut cards = vec![1; winners.len()];
    for  idx in 0..cards.len() {
        let count = winners[idx];
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
        assert_eq!(part_one(input), 32609);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 14624680);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 13);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 30);
    }
}
