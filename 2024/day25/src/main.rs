fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    use std::collections::HashSet;

    let (locks, keys) = input.split("\n\n")
        .fold((HashSet::new(), HashSet::new()), |(mut locks, mut keys), s| {
            let mut hts = [0; 5];
            let lock = s.starts_with('#');
            s.lines()
                .enumerate()
                .for_each(|(r, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '#')
                        .for_each(|(i, _)| {
                            if lock {
                                hts[i] = r
                            } else {
                                hts[i] = std::cmp::max(hts[i], 6 - r);
                            }
                        })
                });
            if lock { locks.insert(hts); } else { keys.insert(hts); }
            (locks, keys)
        });

    locks.iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| {
                    lock.iter()
                        .zip(key.iter())
                        .all(|(l, k)| *l + *k <= 5)
                })
                .count()
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 3127);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 3);
    }
}
