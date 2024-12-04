fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    let mut ncols = 0;
    let m = input.lines()
        .fold(vec![], |mut v, line| {
            ncols = line.len();
            v.extend(line.chars());
            v
        });

    m.iter()
        .enumerate()
        .filter(|(_, c)| **c == 'X')
        .map(|(i, _)| {
            lt(&m, i, ncols) +
            rt(&m, i, ncols) +
            up(&m, i, ncols) +
            dn(&m, i, ncols) +
            up_lt(&m, i, ncols) +
            up_rt(&m, i, ncols) +
            dn_lt(&m, i, ncols) +
            dn_rt(&m, i, ncols)
        })
        .sum()
}

fn part_two(input: &str) -> usize
{
    let mut ncols = 0;
    let m = input.lines()
        .fold(vec![], |mut v, line| {
            ncols = line.len();
            v.extend(line.chars());
            v
        });

    m.iter()
        .enumerate()
        .filter(|(_, c)| **c == 'A')
        .filter_map(|(i, _)| x_mas(&m, i, ncols).then_some(0))
        .count()
}

fn lt(m: &[char], pos: usize, ncols: usize) -> u32
{
    let col = pos % ncols;

    (
        col > 2 &&
        m[pos - 1] == 'M' &&
        m[pos - 2] == 'A' &&
        m[pos - 3] == 'S'
    ) as u32
}

fn rt(m: &[char], pos: usize, ncols: usize) -> u32
{
    let col = pos % ncols;

    (
        col < ncols - 3 &&
        m[pos + 1] == 'M' &&
        m[pos + 2] == 'A' &&
        m[pos + 3] == 'S'
    ) as u32
}

fn up(m: &[char], pos: usize, ncols: usize) -> u32
{
    let row = pos / ncols;
    let offset = ncols;

    (
        row > 2 &&
        m[pos - offset] == 'M' &&
        m[pos - 2 * offset] == 'A' &&
        m[pos - 3 * offset] == 'S'
    ) as u32
}

fn dn(m: &[char], pos: usize, ncols: usize) -> u32
{
    let row    = pos / ncols;
    let nrows  = m.len() / ncols;
    let offset = ncols;

    (
        row < nrows - 3 &&
        m[pos + offset] == 'M' &&
        m[pos + 2 * offset] == 'A' &&
        m[pos + 3 * offset] == 'S'
    ) as u32
}

fn up_lt(m: &[char], pos: usize, ncols: usize) -> u32
{
    let col = pos % ncols;
    let row = pos / ncols;
    let offset = ncols + 1;

    (
        col > 2 && 
        row > 2 &&
        m[pos - offset] == 'M' &&
        m[pos - 2 * offset] == 'A' &&
        m[pos - 3 * offset] == 'S'
    ) as u32
}

fn up_rt(m: &[char], pos: usize, ncols: usize) -> u32
{
    let col    = pos % ncols;
    let row    = pos / ncols;
    let offset = ncols - 1;

    (
        col < ncols - 3 &&
        row > 2 &&
        m[pos - offset] == 'M' &&
        m[pos - 2 * offset] == 'A' &&
        m[pos - 3 * offset] == 'S'
    ) as u32
}

fn dn_lt(m: &[char], pos: usize, ncols: usize) -> u32
{
    let col = pos % ncols;
    let row = pos / ncols;
    let nrows  = m.len() / ncols;
    let offset = ncols - 1;

    (
        col > 2 &&
        row < nrows - 3 &&
        m[pos + offset] == 'M' &&
        m[pos + 2 * offset] == 'A' &&
        m[pos + 3 * offset] == 'S'
    ) as u32
}

fn dn_rt(m: &[char], pos: usize, ncols: usize) -> u32
{
    let col    = pos % ncols;
    let row    = pos / ncols;
    let nrows  = m.len() / ncols;
    let offset = ncols + 1;

    (
        col < ncols - 3 &&
        row < nrows - 3 &&
        m[pos + offset] == 'M' &&
        m[pos + 2 * offset] == 'A' &&
        m[pos + 3 * offset] == 'S'
    ) as u32
}

fn x_mas(m: &[char], pos: usize, ncols: usize) -> bool
{
    let col = pos % ncols;
    let row = pos / ncols;
    let nrows = m.len() / ncols;

    col > 0 &&
    row > 0 &&
    col < ncols - 1 &&
    row < nrows - 1 && (
        (m[pos - ncols - 1] == 'M' && m[pos + ncols + 1] == 'S') ||
        (m[pos - ncols - 1] == 'S' && m[pos + ncols + 1] == 'M')
    ) && (
        (m[pos - ncols + 1] == 'M' && m[pos + ncols - 1] == 'S') ||
        (m[pos - ncols + 1] == 'S' && m[pos + ncols - 1] == 'M')
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
