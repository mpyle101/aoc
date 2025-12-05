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

fn part_one(input: &str) -> usize
{
    let (s, ids) = input.split_once("\n\n").unwrap();
    let v = s.lines()
        .map(|l| {
            let (s1, s2) = l.split_once('-').unwrap();
            let l = s1.parse::<u64>().unwrap();
            let h = s2.parse::<u64>().unwrap();

            (l, h)
        })
        .collect::<Vec<_>>();

    ids.lines()
        .flat_map(|l| l.parse::<u64>())
        .filter(|n| v.iter().any(|(l, h)| n >= l && n <= h))
        .count()
}

fn part_two(input: &str) -> usize
{
    let (s, _) = input.split_once("\n\n").unwrap();
    let mut v = s.lines()
        .map(|l| {
            let (s1, s2) = l.split_once('-').unwrap();
            let l = s1.parse::<u64>().unwrap();
            let h = s2.parse::<u64>().unwrap();

            (l, h)
        })
        .collect::<Vec<_>>();
    v.sort();

    let mut ranges = vec![];
    let mut curr = v[0];
    v.iter()
        .skip(1)
        .for_each(|&(l, h)| {
            if l <= curr.1 {
                curr.1 = curr.1.max(h);
            } else {
                ranges.push(curr);
                curr = (l, h)
            }
        });
    ranges.push(curr);

    ranges.iter()
        .map(|&(l, h)| (l..=h).count())
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 643);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 342018167474526);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 3);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 14);
    }
}
