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
    let gifts = load(input);
    gifts.iter()
        .map(|v| {
            let a = [v[0]*v[1], v[1]*v[2], v[0]*v[2]];
            (2 * a.iter().sum::<u32>()) + a.iter().min().unwrap()
        })
        .sum()
}

fn part_two(input: &str) -> u32
{
    let gifts = load(input);
    gifts.iter()
        .map(|t| (t[0]*t[1]*t[2], [2*(t[0]+t[1]), 2*(t[1]+t[2]), 2*(t[0]+t[2])]))
        .map(|(p, v)| p + v.iter().min().unwrap())
        .sum()
}

fn load(input: &str) -> Vec<[u32;3]>
{
    input.lines()
        .map(|s| s.split('x'))
        .map(|mut iter| [
            iter.next().unwrap().parse::<u32>().unwrap(),
            iter.next().unwrap().parse::<u32>().unwrap(),
            iter.next().unwrap().parse::<u32>().unwrap(),
        ])
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1586300);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 3737498);
    }
}