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

fn part_one(input: &str) -> u32
{
    use std::collections::HashSet;

    let tm = Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap();
    let pos = tm.items().position(|(_, v)| *v == b'S').unwrap();

    (0..tm.rows)
        .step_by(2)
        .fold((0, HashSet::from([pos])), |(mut n, beams), r| {
            let mut v = HashSet::new();
            beams.iter()
                .for_each(|c| {
                    if tm[(r, *c)] == b'^' {
                        v.insert(c - 1);
                        v.insert(c + 1);
                        n += 1
                    } else {
                        v.insert(*c);
                    }
                });
            (n, v)
        })
        .0
}

fn part_two(input: &str) -> u64
{
    use std::collections::HashMap;

    let tm = Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap();
    let pos = tm.items().position(|(_, v)| *v == b'S').unwrap();

    (0..tm.rows)
        .step_by(2)
        .fold(HashMap::from([(pos, 1u64)]), |beams, r| {
            let mut m = HashMap::new();
            beams.iter()
                .for_each(|(c, n)| {
                    if tm[(r, *c)] == b'^' {
                        *m.entry(c - 1).or_default() += n;
                        *m.entry(c + 1).or_default() += n;
                    } else {
                        *m.entry(*c).or_default() += n;
                    }
                });
            m
        })
        .values()
        .sum()

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1656);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 76624086587804);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 21);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 40);
    }
}
