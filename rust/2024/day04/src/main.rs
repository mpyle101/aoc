fn main()
{
    use std::time::Instant;

    let input = include_bytes!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &[u8]) -> u32
{
    let mut ncols = 0;
    let puzzle = input.split(|b| b == &b'\n')
        .fold(Vec::with_capacity(140*140), |mut v, line| {
            ncols = line.len();
            v.extend(line);
            v
        });

    let nrows = puzzle.len() / ncols;
    puzzle.iter()
        .enumerate()
        .filter(|(_, c)| **c == b'X')
        .map(|(i, _)| {
            let col = i % ncols;
            let row = i / ncols;

            lt(&puzzle, i, col) +
            rt(&puzzle, i, col, ncols) +
            up(&puzzle, i, row, ncols) +
            dn(&puzzle, i, row, nrows, ncols) +
            up_lt(&puzzle, i, row, col, ncols) +
            up_rt(&puzzle, i, row, col, ncols) +
            dn_lt(&puzzle, i, row, col, nrows, ncols) +
            dn_rt(&puzzle, i, row, col, nrows, ncols)
        })
        .sum()
}

fn part_two(input: &[u8]) -> usize
{
    let mut ncols = 0;
    let puzzle = input.split(|b| b == &b'\n')
        .fold(Vec::with_capacity(140*140), |mut v, line| {
            ncols = line.len();
            v.extend(line);
            v
        });

    let nrows = puzzle.len() / ncols;
    puzzle.iter()
        .enumerate()
        .filter(|(i, c)| **c == b'A' && x_mas(&puzzle, *i, nrows, ncols))
        .count()
}

fn lt(v: &[u8], i: usize, col: usize) -> u32
{
    (
        col > 2 &&
        v[i - 1] == b'M' &&
        v[i - 2] == b'A' &&
        v[i - 3] == b'S'
    ) as u32
}

fn rt(v: &[u8], i: usize, col: usize, ncols: usize) -> u32
{
    (
        col < ncols - 3 &&
        v[i + 1] == b'M' &&
        v[i + 2] == b'A' &&
        v[i + 3] == b'S'
    ) as u32
}

fn up(v: &[u8], i: usize, row: usize, ncols: usize) -> u32
{
    (
        row > 2 &&
        v[i - ncols]     == b'M' &&
        v[i - 2 * ncols] == b'A' &&
        v[i - 3 * ncols] == b'S'
    ) as u32
}

fn dn(v: &[u8], i: usize, row: usize, nrows: usize, ncols: usize) -> u32
{
    (
        row < nrows - 3 &&
        v[i + ncols]     == b'M' &&
        v[i + 2 * ncols] == b'A' &&
        v[i + 3 * ncols] == b'S'
    ) as u32
}

fn up_lt(v: &[u8], i: usize, row: usize, col: usize, ncols: usize) -> u32
{
    let offset = ncols + 1;
    (
        col > 2 && row > 2 &&
        v[i - offset]     == b'M' &&
        v[i - 2 * offset] == b'A' &&
        v[i - 3 * offset] == b'S'
    ) as u32
}

fn up_rt(v: &[u8], i: usize, row: usize, col: usize, ncols: usize) -> u32
{
    let offset = ncols - 1;
    (
        col < ncols - 3 && row > 2 &&
        v[i - offset]     == b'M' &&
        v[i - 2 * offset] == b'A' &&
        v[i - 3 * offset] == b'S'
    ) as u32
}

fn dn_lt(v: &[u8], i: usize, row: usize, col: usize, nrows: usize, ncols: usize) -> u32
{
    let offset = ncols - 1;
    (
        col > 2 && row < nrows - 3 &&
        v[i + offset]     == b'M' &&
        v[i + 2 * offset] == b'A' &&
        v[i + 3 * offset] == b'S'
    ) as u32
}

fn dn_rt(v: &[u8], i: usize, row: usize, col: usize, nrows: usize, ncols: usize) -> u32
{
    let offset = ncols + 1;
    (
        col < ncols - 3 && row < nrows - 3 &&
        v[i + offset]     == b'M' &&
        v[i + 2 * offset] == b'A' &&
        v[i + 3 * offset] == b'S'
    ) as u32
}

fn x_mas(v: &[u8], i: usize, nrows: usize, ncols: usize) -> bool
{
    let col = i % ncols;
    let row = i / ncols;

    col > 0 && row > 0 &&
    col < ncols - 1 &&
    row < nrows - 1 && (
        (v[i - ncols - 1] == b'M' && v[i + ncols + 1] == b'S') ||
        (v[i - ncols - 1] == b'S' && v[i + ncols + 1] == b'M')
    ) && (
        (v[i - ncols + 1] == b'M' && v[i + ncols - 1] == b'S') ||
        (v[i - ncols + 1] == b'S' && v[i + ncols - 1] == b'M')
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_bytes!("../input.txt");
        assert_eq!(part_one(input), 2613);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_bytes!("../input.txt");
        assert_eq!(part_two(input), 1905);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_bytes!("../example.txt");
        assert_eq!(part_one(input), 18);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_bytes!("../example.txt");
        assert_eq!(part_two(input), 9);
    }
}
