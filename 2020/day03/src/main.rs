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
    let (nrows, ncols, map) = load(input);
    (0..nrows)
        .map(|row| row * ncols + (row * 3) % ncols)
        .filter(|p| map[*p] == '#')
        .count()
}

fn part_two(input: &str) -> usize
{
    let slopes: [(usize, usize);5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let (nrows, ncols, map) = load(input);
    slopes.iter()
        .map(|(dc, dr)| {
            let (mut row, mut col, mut trees) = (0, 0, 0);
            while row < nrows {
                let p = row * ncols + (col % ncols);
                row += dr; col += dc;
                trees += (map[p] == '#') as usize;
            }
            trees
        })
        .product()
}

fn load(input: &str) -> (usize, usize, Vec<char>)
{
    let mut ncols = 0;
    let mut nrows = 0;
    let map = input.lines()
        .enumerate()
        .fold(vec![], |mut v, (row, line)| {
            nrows = row + 1;
            ncols = line.len();
            v.extend(line.chars());
            v
        });

    (nrows, ncols, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 259);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 2224913600);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 7);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 336);
    }

}