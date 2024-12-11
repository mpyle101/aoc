use std::collections::HashMap;

type Memos = HashMap<(u64, usize), usize>;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input, 75);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    let mut stones = input.split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<_>>();

    (0..25).for_each(|_| stones = blink(&mut stones));

    stones.len()
}

fn part_two(input: &str, blinks: usize) -> usize
{
    let stones = input.split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let mut memos = Memos::new();
    stones.iter()
        .map(|&n| expand(n, blinks, &mut memos))
        .sum()
}

fn blink(stones: &mut [u64]) -> Vec<u64>
{
    stones.iter()
        .fold(vec![], |mut v, &n| {
            if n == 0 {
                v.push(1);
            } else if let Some((lt, rt)) = split(n) {
                v.push(lt);
                v.push(rt);
            } else {
                v.push(n * 2024);
            }
            v
        })
}

fn expand(n: u64, blinks: usize, m: &mut Memos) -> usize
{
    if let Some(count) = m.get(&(n, blinks)) {
        return *count
    }

    if blinks == 1 {
        return if n == 0 {
            1
        } else if split(n).is_some() { 
            2
        } else {
            1
        };
    }

    let count = if n == 0 {
        expand(1, blinks - 1, m)
    } else if let Some((lt, rt)) = split(n) {
        expand(lt, blinks - 1, m) +
        expand(rt, blinks - 1, m)
    } else {
        expand(n * 2024, blinks - 1, m)
    };
    m.insert((n, blinks), count);

    count
}

fn split(n: u64) -> Option<(u64, u64)>
{
    let digits = n.ilog10() + 1;
    if digits % 2 == 0 {
        let lt = n / 10_u64.pow(digits / 2);
        let rt = n - lt * 10_u64.pow(digits / 2);
        Some((lt, rt))
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 199753);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input, 75), 239413123020116);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 55312);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input, 25), 55312);
    }
}
