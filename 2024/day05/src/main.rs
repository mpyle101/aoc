use std::collections::HashMap;

type Rules = HashMap<u32, Vec<u32>>;

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

fn part_one(input: &str) -> u32
{
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let mut rules = rules.lines()
        .filter_map(|line| line.split_once('|'))
        .fold(Rules::new(), |mut m, (a, b)| {
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            m.entry(a).or_default().push(b);
            m
        });
    rules.iter_mut()
        .for_each(|(_, v)| v.sort_unstable());

    pages.lines()
        .map(|line| line.split(',')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>())
        .filter(|v| is_ordered(&rules, v))
        .map(|v| v[v.len() / 2])
        .sum()
}

fn part_two(input: &str) -> u32
{
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let mut rules = rules.lines()
        .filter_map(|line| line.split_once('|'))
        .fold(Rules::new(), |mut m, (a, b)| {
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            m.entry(a).or_default().push(b);
            m
        });
    rules.iter_mut()
        .for_each(|(_, v)| v.sort_unstable());

    pages.lines()
        .map(|line| line.split(',')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>())
        .filter_map(|mut v| is_reordered(&rules, &mut v).then_some(v) )
        .map(|v| v[v.len() / 2])
        .sum()
}

fn is_ordered(rules: &Rules, v: &[u32]) -> bool
{
    use utils::ix;

    for (i, j) in ix::from(v) {
        if let Some(r) = rules.get(&v[j]) {
            if r.binary_search(&v[i]).is_ok() {
                return false
            }
        }
    }

    true
}

fn is_reordered(rules: &Rules, v: &mut [u32]) -> bool
{
    use utils::ix;

    let mut reordered = false;

    for (i, j) in ix::from(v) {
        if let Some(r) = rules.get(&v[j]) {
            if r.binary_search(&v[i]).is_ok() {
                v.swap(i, j);
                reordered = true;
            }
        }
    }

    reordered
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 4281);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 5466);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 143);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 123);
    }
}
