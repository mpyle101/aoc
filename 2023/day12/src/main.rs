use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
    use rayon::prelude::*;

    input.par_lines()
        .map(|line| {
            let (springs, groups) = parse_record(line);
            arrangements(springs, &groups)
        })
        .sum()
}

fn part_two(input: &str) -> u64
{
    use rayon::prelude::*;

    input.par_lines()
        .map(|line| {
            let (s, g) = parse_record(line);
            let springs = (0..5)
                .map(|_| s)
                .collect::<Vec<_>>()
                .join("?");
            let groups = (0..5)
                .flat_map(|_| g.clone())
                .collect::<Vec<_>>();

            arrangements(&springs, &groups)
        })
        .sum()
}

fn parse_record(line: &str) -> (&str, Vec<u32>)
{
    let (springs, g) = line.split_once(' ').unwrap();
    let groups = g.split(',')
        .flat_map(|n| n.parse())
        .collect();

    (springs, groups)
}

fn arrangements(springs: &str, groups: &[u32]) -> u64
{
    let mut cache = HashMap::new();
    count(springs, groups, &mut vec![0], &mut cache)
}

fn count<'a>(
    springs: &'a str,
    groups: &[u32],
    found: &mut Vec<u32>,
    cache: &mut HashMap<(&'a str, u64), u64>
) -> u64
{
    let h = calculate_hash(found);
    if let Some(n) = cache.get(&(springs, h)) {
        return *n;
    }

    let i = found.len() - 1;

    if !check(groups, found) {
        return 0;
    } else if springs.is_empty() {
        if found[i] == 0 {
            return (groups == &found[..i]) as u64
        } else {
            return (groups == found) as u64
        }
    }

    let c = springs.chars().next().unwrap();
    let mut n = 0;

    if c == '.' {
        if found[i] == 0 {
            n += count(&springs[1..], groups, found, cache)
        } else {
            found.push(0);
            n += count(&springs[1..], groups, found, cache)
        }
    } else if c == '#' {
        found[i] += 1;
        n += count(&springs[1..], groups, found, cache)
    } else {
        // as '#'
        let mut v = found.clone(); v[i] += 1;
        n += count(&springs[1..], groups, &mut v, cache);
 
        // as '.'
        if found[i] == 0 {
            n += count(&springs[1..], groups, found, cache)
        } else {
            found.push(0);
            n += count(&springs[1..], groups, found, cache)
        }
    }

    cache.insert((springs, h), n);

    n
}

fn check(groups: &[u32], found: &[u32]) -> bool
{
    let i = found.len() - 1;

    if found.len() > groups.len() {
        return found[i] == 0;
    }
    if found[i] > groups[i] {
        return false
    }

    found[..i].iter().zip(groups.iter()).all(|(a, b)| a == b)
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 7307);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 3_415_570_893_842);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 21);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 525152);
    }
}
