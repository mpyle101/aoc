use std::collections::HashMap;

type Antennas = HashMap<char, Vec<(i32, i32)>>;

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
    use utils::ix;
    use std::collections::HashSet;

    let mut nrows = 0;
    let mut ncols = 0;

    let antennas = input.lines()
        .zip(0..)
        .fold(Antennas::new(), |mut m, (line, row)| {
            nrows += 1;
            ncols = line.len() as i32;
            line.chars()
                .zip(0..)
                .for_each(|(c, col)| {
                    if c != '.' {
                        m.entry(c).or_default().push((row, col))
                    }
                });
            m
        });

    let locations = antennas.values()
        .fold(HashSet::new(), |mut acc, v| {
            for (i, j) in ix::from(v.len()) {
                let dr = v[i].0 - v[j].0;
                let dc = v[i].1 - v[j].1;

                let p = (v[i].0 + dr, v[i].1 + dc);
                if is_inbounds(p, nrows, ncols) {
                    acc.insert(p);
                }
                let p = (v[j].0 - dr, v[j].1 - dc);
                if is_inbounds(p, nrows, ncols) {
                    acc.insert(p);
                }
            }
            acc
        });

    locations.len()
}

fn part_two(input: &str) -> usize
{
    use utils::ix;
    use std::collections::HashSet;

    let mut nrows = 0;
    let mut ncols = 0;

    let antennas = input.lines()
        .zip(0..)
        .fold(Antennas::new(), |mut m, (line, row)| {
            nrows += 1;
            ncols = line.len() as i32;
            line.chars()
                .zip(0..)
                .for_each(|(c, col)| {
                    if c != '.' {
                        m.entry(c).or_default().push((row, col))
                    }
                });
            m
        });

    let locations = antennas.values()
        .fold(HashSet::new(), |mut acc, v| {
            for (i, j) in ix::from(v.len()) {
                let dr = v[i].0 - v[j].0;
                let dc = v[i].1 - v[j].1;

                let mut p = (v[i].0 - dr, v[i].1 - dc);
                while is_inbounds(p, nrows, ncols) {
                    acc.insert(p);
                    p = (p.0 + dr, p.1 + dc)
                }

                let mut p = (v[j].0 + dr, v[j].1 + dc);
                while is_inbounds(p, nrows, ncols) {
                    acc.insert(p);
                    p = (p.0 - dr, p.1 - dc)
                }
            }
            acc
        });

    locations.len()
}

fn is_inbounds((row, col): (i32, i32), nrows: i32, ncols: i32) -> bool
{
    row > -1 && col > -1 && row < nrows && col < ncols
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 247);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 861);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 14);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 34);
    }
}
