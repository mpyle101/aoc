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
    let grid = input.lines()
        .inspect(|l| { nrows += 1; ncols = l.len() })
        .fold(Vec::new(), |mut v, l| { v.extend_from_slice(l.as_bytes()); v });

    (0..100)
        .fold(grid, |g, _| {
            g.iter()
                .enumerate()
                .map(|(i, c)| {
                    let lit = neighbours(i, nrows, ncols)
                        .filter(|ix| g[*ix] == b'#')
                        .count();
                    (c, lit)
                })
                .map(|(c, lit)| {
                    match (c, lit) {
                        (b'.', 3) => b'#',
                        (b'#', 2) => b'#',
                        (b'#', 3) => b'#',
                        _         => b'.'
                    }
                })
                .collect()
        })
        .iter()
        .filter(|c| **c == b'#')
        .count()
}

fn part_two(input: &str) -> usize
{
    let mut nrows = 0;
    let mut ncols = 0;
    let mut grid = input.lines()
        .inspect(|l| { nrows += 1; ncols = l.len() })
        .fold(Vec::new(), |mut v, l| { v.extend_from_slice(l.as_bytes()); v });

    // top left, top right, bottom left, bottom right
    let on = [0, ncols - 1, (nrows - 1) * ncols, nrows * ncols - 1];
    on.iter().for_each(|i| grid[*i] = b'#');

    (0..100)
        .fold(grid, |g, _| {
            let mut grid = g.iter()
                .enumerate()
                .map(|(i, c)| {
                    let lit = neighbours(i, nrows, ncols)
                        .filter(|ix| g[*ix] == b'#')
                        .count();
                    (c, lit)
                })
                .map(|(c, lit)| {
                    match (c, lit) {
                        (b'.', 3) => b'#',
                        (b'#', 2) => b'#',
                        (b'#', 3) => b'#',
                        _         => b'.'
                    }
                })
                .collect::<Vec<_>>();
            on.iter().for_each(|i| grid[*i] = b'#');
            grid
        })
        .iter()
        .filter(|c| **c == b'#')
        .count()
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
        assert_eq!(part_one(input), 1061);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1006);
    }
}
