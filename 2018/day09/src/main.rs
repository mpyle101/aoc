
fn main()
{
    use std::time::Instant;

    let input = include_str!("./input.txt");

    let t = Instant::now();
    println!("Part 1: {}  ({:?})", part_one(input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {}  ({:?})", part_two(input), t.elapsed());
}

fn part_one(input: &str) -> u32
{
    let (players, last_marble) = load(input);
    doit(players, last_marble)
}

fn part_two(input: &str) -> u32
{
    let (players, last_marble) = load(input);
    doit(players, last_marble * 100)
}

fn load(input: &str) -> (u32, u32)
{
    let v: Vec<_> = input.split(' ').collect();

    (v[0].parse::<u32>().unwrap(), v[6].parse::<u32>().unwrap())
}

fn doit(players: u32, last_marble: u32) -> u32
{
    use std::collections::{HashMap, VecDeque};

    let mut scores = HashMap::new();
    let mut q = VecDeque::new();

    // Double ended queue with the current marble always at the end.
    q.push_back(0);
    for m in 1..last_marble + 1 {
        if m % 23 == 0 {
            q.rotate_right(7);
            *scores.entry(m % players).or_insert(0) += m + q.pop_back().unwrap();
            q.rotate_left(1);
        } else {
            q.rotate_left(1);
            q.push_back(m);
        }
    }

    *scores.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("./input.txt");
        assert_eq!(part_one(input), 375465);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("./input.txt");
        assert_eq!(part_two(input), 3037741441);
    }
}
