use std::collections::{HashMap, HashSet};

type Memo<'a> = HashMap<&'a str, u32>;
type Orbits<'a> = HashMap<&'a str, &'a str>;
type Transfers<'a> = HashMap<&'a str, Vec<&'a str>>;

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
    let orbits = load_orbits(input);
    let mut counts = Memo::from([("COM", 0)]);
    orbits.keys().for_each(|obj| { to_com(obj, &orbits, &mut counts); });

    counts.values().sum()
}

#[allow(non_snake_case)]
fn part_two(input: &str) -> u32
{
    let xfers = load_xfers(input);
    let YOU = xfers.get("YOU").map(|v| v[0]).unwrap();
    let SAN = xfers.get("SAN").map(|v| v[0]).unwrap();

    let mut visited = HashSet::from(["YOU", "SAN"]);
    to_santa(YOU, SAN, &xfers, &mut visited)
}

fn to_com<'a>(obj: &'a str, orbits: &Orbits<'a>, counts: &mut Memo<'a>) -> u32
{
    if let Some(n) = counts.get(obj) {
        *n
    } else if let Some(o) = orbits.get(obj) {
        let n = to_com(o, orbits, counts) + 1;
        *counts.entry(obj).or_default() = n;
        n
    } else {
        0
    }
}

fn to_santa<'a>(obj: &'a str, san: &'a str, xfers: &Transfers<'a>, visited: &mut HashSet<&'a str>) -> u32
{
    if obj == san {
        0
    } else if let Some(v) = xfers.get(obj) {
        // Can't use u32::MAX because it will rollover when we add 1.
        let mut n = 10_000_000;
        for o in v {
            if visited.insert(o) {
                n = n.min(1 + to_santa(o, san, xfers, visited));
            }
        }
        n
    } else {
        10_000_000
    }
}

fn load_orbits(input: &str) -> Orbits
{
    input.lines()
        .map(|line| {
            let (s1, s2) = line.split_once(')').unwrap();
            (s2, s1)
        })
        .collect()
}

fn load_xfers(input: &str) -> Transfers
{
    input.lines()
        .fold(Transfers::new(), |mut m, line| {
            let (s1, s2) = line.split_once(')').unwrap();
            m.entry(s1).or_default().push(s2);
            m.entry(s2).or_default().push(s1);
            m
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 245089);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 511);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 42);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example2.txt");
        assert_eq!(part_two(input), 4);
    }
}