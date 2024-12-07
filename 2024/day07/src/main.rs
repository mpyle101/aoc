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

fn part_one(input: &str) -> u64
{
    let ops = [
        |a: u64, b: u64| a + b,
        |a: u64, b: u64| a * b
    ];

    input.lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(s1, s2)| {
            let target  = s1.parse::<u64>().unwrap();
            let formula = s2.split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            valid(target, formula[0], &formula[1..], &ops)
                .then_some(target)
        })
        .sum()
}

fn part_two(input: &str) -> u64
{
    let ops = [
        |a: u64, b: u64| a + b,
        |a: u64, b: u64| a * b,
        |a: u64, b: u64| a * 10u64.pow(b.ilog10() + 1) + b
    ];

    input.lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(s1, s2)| {
            let target  = s1.parse::<u64>().unwrap();
            let formula = s2.split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            valid(target, formula[0], &formula[1..], &ops)
                .then_some(target)
        })
        .sum()
}

fn valid(target: u64, partial: u64, v: &[u64], ops: &[fn(u64, u64) -> u64]) -> bool
{
    if partial > target {
        false
    } else if v.is_empty() {
        partial == target
    } else {
        ops.iter().any(|f| valid(target, f(partial, v[0]), &v[1..], ops))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 2941973819040);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 249943041417600);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 3749);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 11387);
    }
}
