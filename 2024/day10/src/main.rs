use std::collections::HashSet;

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
    let (start, map, nrows, ncols) = load(input);

    start.iter()
        .map(|pos| score(*pos, &map, nrows, ncols, &mut HashSet::new()))
        .sum()
}

fn part_two(input: &str) -> usize
{
    let (start, map, nrows, ncols) = load(input);

    start.iter()
        .map(|pos| rating(*pos, &map, nrows, ncols))
        .sum()
}

fn load(input: &str) -> (Vec<usize>, Vec<u8>, usize, usize)
{
    let mut ncols = 0;
    let mut nrows = 0;
    let mut start = vec![];
    let map = input.lines()
        .enumerate()
        .fold(vec![], |mut v, (row, line)| {
            nrows = row + 1;
            ncols = line.len();

            v.extend(line.bytes()
                .enumerate()
                .map(|(col, c)| {
                    let height = c - b'0';
                    if height == 0 {
                        start.push(row * ncols + col);
                    }
                    height
                }));
            v
        });

    (start, map, nrows, ncols)
}

fn score(pos: usize, map: &[u8], nrows: usize, ncols: usize, visited: &mut HashSet<usize>) -> usize
{
    if map[pos] == 9 {
        visited.insert(pos) as usize
    } else {
        let v = moves(pos, map, nrows, ncols);
        if v.is_empty() {
            0
        } else {
            v.iter()
                .map(|p| score(*p, map, nrows, ncols, visited))
                .sum()
        }
    }
}

fn rating(pos: usize, map: &[u8], nrows: usize, ncols: usize) -> usize
{
    if map[pos] == 9 {
        1
    } else {
        let v = moves(pos, map, nrows, ncols);
        if v.is_empty() {
            0
        } else {
            v.iter()
                .map(|p| rating(*p, map, nrows, ncols))
                .sum()
        }
    }
}

fn moves(pos: usize, map: &[u8], nrows: usize, ncols: usize) -> Vec<usize>
{
    let mut v = vec![];

    let n = map[pos] + 1;
    let row = pos / ncols;
    let col = pos % ncols;

    if col > 0 && map[pos - 1] == n {
        v.push(pos - 1);
    }
    if row > 0 && map[pos - ncols] == n {
        v.push(pos - ncols);
    }
    if col < ncols - 1 && map[pos + 1] == n {
        v.push(pos + 1);
    }
    if row < nrows - 1 && map[pos + ncols] == n {
        v.push(pos + ncols);
    }

    v
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 746);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1541);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 36);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 81);
    }
}
