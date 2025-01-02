use std::collections::{HashMap, HashSet};

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
    let check = |&n: &usize| n == 2 || n == 3 || n == 4 || n == 7;

    input.lines()
        .flat_map(|line| line.split_once(" | "))
        .map(|(_, s)| s.split(' '))
        .map(|sp| sp.map(|s| s.len()).filter(check).count())
        .sum()
}

fn part_two(input: &str) -> u32
{
    let to_char = |set: HashSet<char>| *set.iter().next().unwrap();
    let digits: HashMap<String, u32> = HashMap::from([
        ("abcefg".into(),  0),
        ("cf".into(),      1),
        ("acdeg".into(),   2),
        ("acdfg".into(),   3),
        ("bcdf".into(),    4),
        ("abdfg".into(),   5),
        ("abdefg".into(),  6),
        ("acf".into(),     7),
        ("abcdefg".into(), 8),
        ("abcdfg".into(),  9),
    ]);

    input.lines()
        .flat_map(|line| line.split_once(" | "))
        .map(|(s1, s2)| (s1.split(' '), s2.split(' ')))
        .map(|(sp1, sp2)| {
            // Note, each line contains all ten unique signal patterns
            // on the left side.
            let v = segments(sp1);
            let cf      = &v[2][0];     // 1
            let acf     = &v[3][0];     // 7
            let bcdf    = &v[4][0];     // 4
            let abcdefg = &v[7][0];     // 8

            let a      = acf - cf;
            let bd     = bcdf - cf;
            let abcdf  = bcdf | &a;
            let acdeg  = v[5].iter().find(|&s| (s - &abcdf).len() == 2).unwrap();
            let abcdfg = v[6].iter().find(|&s| (s - &abcdf).len() == 1).unwrap();
            let b      = &bd - acdeg;
            let d      = &bd - &b;
            let e      = abcdefg - abcdfg;
            let f      = &abcdf - &(&bd | acdeg);
            let g      = abcdfg - &abcdf;
            let c      = cf - &f;

            let keys = HashMap::from([
                (to_char(a), 'a'),
                (to_char(b), 'b'),
                (to_char(c), 'c'),
                (to_char(d), 'd'),
                (to_char(e), 'e'),
                (to_char(f), 'f'),
                (to_char(g), 'g'),
            ]);

            sp2.map(|s| {
                    let mut v = s.chars()
                        .flat_map(|c| keys.get(&c))
                        .collect::<Vec<_>>();
                    v.sort();
                    v.iter().cloned().collect::<String>()
                })
                .zip((0..=3).rev())
                .fold(0, |acc, (s, i)| {
                    let n = digits.get(&s).unwrap();
                    acc + (n * 10u32.pow(i))
                })
        })
        .sum()
}

type Segments = Vec<Vec<HashSet<char>>>;
fn segments<'a>(it: impl Iterator<Item=&'a str>) -> Segments
{
    let mut segments = vec![vec![];8];
    it.map(|s| s.chars().collect::<HashSet<_>>())
        .for_each(|s| segments[s.len()].push(s) );
    segments
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 381);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1023686);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 26);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 61229);
    }
}