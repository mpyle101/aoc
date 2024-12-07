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
    input.lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(s1, s2)| {
            let value = s1.parse::<u64>().unwrap();
            let formula = s2.split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            valid(value, formula[0], &formula[1..]).then_some(value)
        })
        .sum()
}

fn part_two(input: &str) -> u64
{
    input.lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(s1, s2)| {
            let value = s1.parse::<u64>().unwrap();
            let formula = s2.split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            valid2(value, formula[0], &formula[1..]).then_some(value)
        })
        .sum()
}

fn valid(val: u64, partial: u64, v: &[u64]) -> bool
{
    let n = v[0];
    if partial > val {
        false
    } else if v.len() == 1 {
        partial * n == val || partial + n == val
    } else {
        valid(val, partial + n, &v[1..]) ||
        valid(val, partial * n, &v[1..])
    }
}

fn valid2(val: u64, partial: u64, v: &[u64]) -> bool
{
    let n = v[0];
    if partial > val {
        false
    } else if v.len() == 1 {
        partial * n == val ||
        partial + n == val ||
        concat(partial, n) == val
    } else {
        valid2(val, partial + n, &v[1..]) ||
        valid2(val, partial * n, &v[1..]) ||
        valid2(val, concat(partial, n), &v[1..])
    }
}

fn concat(a: u64, b: u64) -> u64
{
    a * 10u64.pow(b.ilog10() + 1) + b
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
