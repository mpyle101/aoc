type Rules<'a> = Vec<(&'a str, &'a str)>;

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
    use std::collections::HashSet;

    let (rules, molecule) = load(input);
    rules.iter()
        .flat_map(|(sequence, replacement)| {
            let n = sequence.len();
            molecule
                .match_indices(sequence)
                .map(move |(i, _)| {
                    let mut m = molecule.to_string();
                    m.replace_range(i..i+n, replacement);
                    m
                })
        })
        .collect::<HashSet<_>>()
        .len()
}

fn part_two(input: &str) -> u32
{
    use rand::prelude::SliceRandom;

    let (mut rules, molecule) = load(input);

    let mut count = 0;
    let mut m = molecule.to_string();

    while m != "e" {
        let n = count;
        for (s, r) in &rules {
            let mut m1 = m.clone();
            m.rmatch_indices(r)
                .for_each(|(i, _)| {
                    count += 1;
                    let n = r.len();
                    m1.replace_range(i..i+n, s);
                });
                m = m1
        }
        if count == n {
            m = molecule.to_string();
            count = 0;
            rules.shuffle(&mut rand::thread_rng());
        }
    }

    count
}

fn load(input: &str) -> (Rules<'_>, &str)
{
    let (rules, molecule) = input.split_once("\n\n").unwrap();
    let rules = rules.lines()
        .flat_map(|line| line.split_once(" => "))
        .collect();

    (rules, molecule)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 576);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 207);
    }
}