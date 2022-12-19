use pathfinding::matrix::Matrix;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let steps = part_one(input);
    println!("Part 1: {} ({:?})", steps, t.elapsed());

    let t = Instant::now();
    let steps = part_two(input);
    println!("Part 2: {} ({:?})", steps, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    use pathfinding::prelude::bfs;

    let (start, end, m) = load(input);
    bfs(&start, |&p| neighbors(p, &m), |&p| p == end).unwrap().len() - 1
}

fn part_two(input: &str) -> usize
{
    use pathfinding::prelude::bfs;

    let (_, end, m) = load(input);
    m.indices()
        .filter_map(|p| m.get(p).and_then(|h| (*h == 0).then_some(p)))
        .filter_map(|p| bfs(&p, |&p| neighbors(p, &m), |&p| p == end))
        .map(|v| v.len() - 1)
        .min()
        .unwrap()
}

fn load(input: &str) -> ((usize, usize), (usize, usize), Matrix<u8>)
{
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut m = Matrix::from_rows(input.lines()
        .map(|line| line.bytes().map(|byte|
            match byte {
                    b'S' => 100,
                    b'E' => 200,
                    b    => b - b'a'
                })
            ))
        .unwrap();

    m.keys().for_each(|p| {
        let v = m.get(p).unwrap();
        if *v == 100 { start = p } else if *v == 200 { end = p }
    });
    *m.get_mut(start).unwrap() = 0;
    *m.get_mut(end).unwrap() = 25;

    (start, end, m)
}

fn neighbors(p: (usize, usize), m: &Matrix<u8>) -> impl Iterator<Item = (usize, usize)> + '_
{
    let h = m.get(p).unwrap();
    m.neighbours(p, false).filter(|pos| (*h + 1) >= *m.get(*pos).unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let steps = part_one(input);
        assert_eq!(steps, 456);

        let steps = part_one(input);
        assert_eq!(steps, 454);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let steps = part_one(input);
        assert_eq!(steps, 31);

        let steps = part_two(input);
        assert_eq!(steps, 29);
    }
}
