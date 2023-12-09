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
    input.lines()
        .map(|line| {
            let seq: Vec<i32> = line.split(' ')
                .flat_map(|s| s.parse())
                .collect();
            extrapolate(&seq)
        })
        .sum()
}

fn part_two(input: &str) -> i32
{
    input.lines()
        .map(|line| {
            let seq: Vec<i32> = line.split(' ')
                .flat_map(|s| s.parse())
                .collect();
            interpolate(&seq)
        })
        .sum()
}

fn extrapolate(seq: &[i32]) -> i32
{
    let mut v = vec![];
    let mut last = vec![seq[seq.len() - 1]];
    let mut diffs = differences(seq);
    while diffs.iter().any(|n| *n != 0) {
        last.push(diffs[diffs.len() - 1]);
        v.push(diffs);
        diffs = differences(&v[v.len() - 1]);
    }

    last.iter().sum()
}

fn interpolate(seq: &[i32]) -> i32
{
    let mut v = vec![];
    let mut first = vec![seq[0]];
    let mut diffs = differences(seq);
    while diffs.iter().any(|n| *n != 0) {
        first.push(diffs[0]);
        v.push(diffs);
        diffs = differences(&v[v.len() - 1]);
    }

    first.iter().rev()
        .cloned()
        .reduce(|acc, n| n - acc)
        .unwrap()
}

fn differences(seq: &[i32]) -> Vec<i32>
{
    (1..seq.len()).map(|i| seq[i] - seq[i-1]).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1939607039);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1041);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 114);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 2);
    }
}
