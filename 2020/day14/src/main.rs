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
    use std::collections::HashMap;

    let mut mask = "";
    let mut memory = HashMap::new();
    input.lines()
        .flat_map(|line| line.split_once(" = "))
        .for_each(|(s1, s2)| {
            if s1 == "mask" {
                mask = s2
            } else {
                let n = s2.parse::<u64>().unwrap();
                let m = s1[4..s1.len()-1].parse::<u32>().unwrap();
                memory.insert(m, apply(mask, n));
            }
        });

    memory.values().sum()
}

fn part_two(input: &str) -> u64
{
    use std::collections::HashMap;

    let mut mask = vec![];
    let mut memory = HashMap::new();
    input.lines()
        .flat_map(|line| line.split_once(" = "))
        .for_each(|(s1, s2)| {
            if s1 == "mask" {
                mask = s2.chars().collect::<Vec<_>>();
            } else {
                let n = s2.parse::<u64>().unwrap();
                let m = s1[4..s1.len()-1].parse::<u64>().unwrap();
                decode(&mask, m)
                    .iter()
                    .for_each(|&a| { memory.insert(a, n); });
            }
        });

    memory.values().sum()
}

fn apply(mask: &str, n: u64) -> u64
{
    mask.chars()
        .rev()
        .enumerate()
        .filter(|(_, c)| *c != 'X')
        .fold(n, |acc, (i, c)| {
            if c == '1' {
                acc | (1 << i)
            } else {
                acc & !(1 << i)
            }
        })
}

fn decode(mask: &[char], m: u64) -> Vec<u64>
{
    if mask.is_empty() {
        vec![m]
    } else {
        let i = mask.len() - 1;
        if mask[0] == '0' {
            decode(&mask[1..], m)
        } else if mask[0] == '1' {
            decode(&mask[1..], m | (1 << i))
        } else {
            let mut v = vec![];
            v.extend(decode(&mask[1..], m | (1 << i)));
            v.extend(decode(&mask[1..], m & !(1 << i)));
            v
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 12512013221615);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 3905642473893);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 165);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example2.txt");
        assert_eq!(part_two(input), 208);
    }
}