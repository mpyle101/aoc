use pathfinding::matrix::Matrix;

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
    use pathfinding::matrix::Matrix;

    let m = Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap();
    m.items()
        .filter(|(_, c)| **c == b'@')
        .filter(|(p, _)| {
            m.neighbours(*p, true)
                .filter(|n| m[n] == b'@')
                .count() < 4
        })
        .count()
}

fn part_two(input: &str) -> usize
{
    use pathfinding::matrix::Matrix;

    let mut m = Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap();

    let mut rolls = 0;
    let mut v = removeable(&m);
    while !v.is_empty() {
        rolls += v.len();
        v.iter().for_each(|p| m[p] = b'.');
        v = removeable(&m);
    }

    rolls
}

fn removeable(m: &Matrix<u8>) -> Vec<(usize, usize)>
{
    m.items()
        .filter(|(_, c)| **c == b'@')
        .filter(|(p, _)| {
            m.neighbours(*p, true)
                .filter(|n| m[n] == b'@')
                .count() < 4
        })
        .map(|(p, _)| p)
        .collect()
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
