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
    let mut ncols = 0;
    let m = input.lines()
        .fold(Vec::with_capacity(140*140), |mut v, line| {
            ncols = line.len();
            v.extend(line.bytes());
            v
        });

    let nrows = m.len() / ncols;
    m.iter()
        .enumerate()
        .filter(|(_, c)| **c == b'X')
        .map(|(i, _)| {
            let col = i % ncols;
            let row = i / ncols;

            lt(&m, i, col) +
            rt(&m, i, col, ncols) +
            up(&m, i, row, ncols) +
            dn(&m, i, row, nrows, ncols) +
            up_lt(&m, i, row, col, ncols) +
            up_rt(&m, i, row, col, ncols) +
            dn_lt(&m, i, row, col, nrows, ncols) +
            dn_rt(&m, i, row, col, nrows, ncols)
        })
        .sum()
}

fn part_two(input: &str) -> usize
{
    let mut ncols = 0;
    let m = input.lines()
        .fold(Vec::with_capacity(140*140), |mut v, line| {
            ncols = line.len();
            v.extend(line.bytes());
            v
        });

    let nrows = m.len() / ncols;
    m.iter()
        .enumerate()
        .filter(|(i, c)| **c == b'A' && x_mas(&m, *i, nrows, ncols))
        .count()
}

fn lt(m: &[u8], pos: usize, col: usize) -> u32
{
    (
        col > 2 &&
        m[pos - 1] == b'M' &&
        m[pos - 2] == b'A' &&
        m[pos - 3] == b'S'
    ) as u32
}

fn rt(m: &[u8], pos: usize, col: usize, ncols: usize) -> u32
{
    (
        col < ncols - 3 &&
        m[pos + 1] == b'M' &&
        m[pos + 2] == b'A' &&
        m[pos + 3] == b'S'
    ) as u32
}

fn up(m: &[u8], pos: usize, row: usize, ncols: usize) -> u32
{
    let offset = ncols;
    (
        row > 2 &&
        m[pos - offset] == b'M' &&
        m[pos - 2 * offset] == b'A' &&
        m[pos - 3 * offset] == b'S'
    ) as u32
}

fn dn(m: &[u8], pos: usize, row: usize, nrows: usize, ncols: usize) -> u32
{
    let offset = ncols;
    (
        row < nrows - 3 &&
        m[pos + offset] == b'M' &&
        m[pos + 2 * offset] == b'A' &&
        m[pos + 3 * offset] == b'S'
    ) as u32
}

fn up_lt(m: &[u8], pos: usize, row: usize, col: usize, ncols: usize) -> u32
{
    let offset = ncols + 1;
    (
        col > 2 && 
        row > 2 &&
        m[pos - offset] == b'M' &&
        m[pos - 2 * offset] == b'A' &&
        m[pos - 3 * offset] == b'S'
    ) as u32
}

fn up_rt(m: &[u8], pos: usize, row: usize, col: usize, ncols: usize) -> u32
{
    let offset = ncols - 1;
    (
        col < ncols - 3 &&
        row > 2 &&
        m[pos - offset] == b'M' &&
        m[pos - 2 * offset] == b'A' &&
        m[pos - 3 * offset] == b'S'
    ) as u32
}

fn dn_lt(m: &[u8], pos: usize, row: usize, col: usize, nrows: usize, ncols: usize) -> u32
{
    let offset = ncols - 1;
    (
        col > 2 &&
        row < nrows - 3 &&
        m[pos + offset] == b'M' &&
        m[pos + 2 * offset] == b'A' &&
        m[pos + 3 * offset] == b'S'
    ) as u32
}

fn dn_rt(m: &[u8], pos: usize, row: usize, col: usize, nrows: usize, ncols: usize) -> u32
{
    let offset = ncols + 1;
    (
        col < ncols - 3 &&
        row < nrows - 3 &&
        m[pos + offset] == b'M' &&
        m[pos + 2 * offset] == b'A' &&
        m[pos + 3 * offset] == b'S'
    ) as u32
}

fn x_mas(m: &[u8], pos: usize, nrows: usize, ncols: usize) -> bool
{
    let col = pos % ncols;
    let row = pos / ncols;

    col > 0 &&
    row > 0 &&
    col < ncols - 1 &&
    row < nrows - 1 && (
        (m[pos - ncols - 1] == b'M' && m[pos + ncols + 1] == b'S') ||
        (m[pos - ncols - 1] == b'S' && m[pos + ncols + 1] == b'M')
    ) && (
        (m[pos - ncols + 1] == b'M' && m[pos + ncols - 1] == b'S') ||
        (m[pos - ncols + 1] == b'S' && m[pos + ncols - 1] == b'M')
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 2613);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1905);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 18);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 9);
    }
}
