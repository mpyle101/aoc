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
    use pathfinding::directed::dfs::dfs;

    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let mut patterns = patterns.split(", ").collect::<Vec<_>>();
    patterns.sort_by(|&a, b| b.len().cmp(&a.len()));

    designs.lines()
        .map(|design| {
            let pat = patterns.iter()
                .filter(|&p| design.contains(p))
                .cloned()
                .collect::<Vec<_>>();
            (design, pat)
        })
        .filter_map(|(design, pat)| {
            dfs(design, |d| possible(d, &pat), |d| d.is_empty())
        })
        .count()
}

fn part_two(input: &str) -> usize
{
    use pathfinding::directed::count_paths::*;

    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let mut patterns = patterns.split(", ").collect::<Vec<_>>();
    patterns.sort_by(|&a, b| b.len().cmp(&a.len()));

    designs.lines()
        .map(|design| {
            let pat = patterns.iter()
                .filter(|&p| design.contains(p))
                .cloned()
                .collect::<Vec<_>>();
            (design, pat)
        })
        .map(|(design, pat)| {
            count_paths(design, |d| possible(d, &pat), |d| d.is_empty())
        })
        .sum()
}

fn possible<'a>(design: &'a str, patterns: &[&str]) -> Vec<&'a str>
{
    patterns.iter()
        .filter(|&pat| design.starts_with(pat))
        .map(|pat| &design[pat.len()..])
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 330);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 950763269786650);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 6);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 16);
    }
}
