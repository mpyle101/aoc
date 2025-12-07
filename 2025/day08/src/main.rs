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

    let mut iter = tm.iter();
    let row = iter.next().unwrap();
    let pos = row.iter().position(|c| *c == b'S').unwrap();

    let mut beams = HashSet::from([pos]);
    (0..tm.rows)
        .step_by(2)
        .fold(0, |mut n, r| {
            let mut v = HashSet::new();
            beams.iter()
                .for_each(|c| {
                    if tm[(r, *c)] == b'^' {
                        v.insert(c-1);
                        v.insert(c+1);
                        n += 1
                    } else {
                        v.insert(*c);
                    }
                });
            beams = v;

            n
        })
}

fn part_two(input: &str) -> u64
{
    use std::collections::HashMap;

    let tm = Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap();

    let mut iter = tm.iter();
    let row = iter.next().unwrap();
    let pos = row.iter().position(|c| *c == b'S').unwrap();

    let beams = (0..tm.rows)
        .step_by(2)
        .fold(HashMap::from([(pos, 1u64)]), |beams, r| {
            let mut m = HashMap::new();
            beams.iter()
                .for_each(|(c, n)| {
                    if tm[(r, *c)] == b'^' {
                        *m.entry(c - 1).or_insert(0) += n;
                        *m.entry(c + 1).or_insert(0) += n;
                    } else {
                        *m.entry(*c).or_insert(0) += n;
                    }
                });
            m
        });

    beams.values().sum()
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
