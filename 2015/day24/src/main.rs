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
    let mut weights = input.lines()
        .flat_map(|line| line.parse::<u32>())
        .collect::<Vec<_>>();
    weights.sort_by(|a, b| b.cmp(a));
    let target = weights.iter().sum::<u32>() / 3;

    let mut v = vec![];
    dfs(2, 0, 0, target, vec![], &weights, &mut v);

    v.iter().map(|n| *n as u64).product::<u64>()
}

fn part_two(input: &str) -> u64
{
    let mut weights = input.lines()
        .flat_map(|line| line.parse::<u32>())
        .collect::<Vec<_>>();
    weights.sort_by(|a, b| b.cmp(a));
    let target = weights.iter().sum::<u32>() / 4;

    let mut v = vec![];
    dfs(3, 0, 0, target, vec![], &weights, &mut v);

    v.iter().map(|n| *n as u64).product::<u64>()
}

fn dfs(m: u32, i: usize, n: u32, t: u32, g: Vec<u32>, weights: &[u32], r: &mut Vec<u32>)
{
    let rl = if r.is_empty() { usize::MAX } else { r.len() };

    if n == t {
        let qe = g.iter().map(|n| *n as u64).product::<u64>();
        let re = r.iter().map(|n| *n as u64).product::<u64>();
        let wts = weights.iter().filter(|w| !g.contains(w)).sum::<u32>();

        if wts == t * m && (r.is_empty() || g.len() < rl || (g.len() == rl && qe < re)) {
            *r = g;
        }
    } else if n < t && i < weights.len() && g.len() < rl {
        let w = weights[i];
        let mut g1 = g.clone(); g1.push(w);
        dfs(m, i + 1, n + w, t, g1, weights, r);
        dfs(m, i + 1, n, t, g, weights, r);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 11846773891);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 80393059);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 99);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 44);
    }
}