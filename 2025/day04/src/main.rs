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
    let mut nrows = 0;
    let mut ncols = 0;
    let v = input.lines()
        .inspect(|l| { nrows += 1; ncols = l.len() })
        .fold(Vec::new(), |mut v, l| { v.extend_from_slice(l.as_bytes()); v });

    v.iter()
        .enumerate()
        .filter(|(_, c)| **c == b'@')
        .filter(|(i, _)| {
            neighbours(*i, nrows, ncols)
                .filter(|&p| v[p] == b'@')
                .count() < 4
        })
        .count()
}

fn part_two(input: &str) -> usize
{
    let mut nrows = 0;
    let mut ncols = 0;
    let mut v = input.lines()
        .inspect(|l| { nrows += 1; ncols = l.len() })
        .fold(Vec::new(), |mut v, l| { v.extend_from_slice(l.as_bytes()); v });

    let mut rolls = 0;
    let mut v1 = removeable(&v, nrows, ncols);
    while !v1.is_empty() {
        rolls += v1.len();
        v1.iter().for_each(|i| v[*i] = b'.');
        v1 = removeable(&v, nrows, ncols);
    }

    rolls
}

fn removeable(v: &[u8], nrows: usize, ncols: usize) -> Vec<usize>
{
    v.iter()
        .enumerate()
        .filter(|(_, c)| **c == b'@')
        .filter(move |(i, _)| {
            neighbours(*i, nrows, ncols)
                .filter(|&p| v[p] == b'@')
                .count() < 4
        })
        .map(|(i, _)| i)
        .collect()
}

fn neighbours(i: usize, nrows: usize, ncols: usize) -> impl Iterator<Item = usize>
{
    let r = i / ncols;
    let c = i % ncols;
    let (rows, cols) = (
        r.saturating_sub(1)..nrows.min(r + 2),
        c.saturating_sub(1)..ncols.min(c + 2)
    );

    rows
        .flat_map(move |rr| cols.clone().map(move |cc| (rr, cc)))
        .filter(move |&p| p != (r, c))
        .map(move |(rr, cc)| rr * ncols + cc)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1356);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 8713);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 13);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 43);
    }
}
