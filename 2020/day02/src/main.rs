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
    input.lines()
        .map(|line| {
            let (s, p) = line.split_once(": ").unwrap();
            let (r, c) = s.split_once(' ').unwrap();
            let (i, j) = r.split_once('-').unwrap();
            let c = c.chars().next().unwrap();
            let i = i.parse::<usize>().unwrap();
            let j = j.parse::<usize>().unwrap();

            (i, j, p.chars().filter(|c1| *c1 == c).count())
        })
        .filter(|(i, j, n)| (*i..=*j).contains(n))
        .count()
}

fn part_two(input: &str) -> usize
{
    input.lines()
        .map(|line| {
            let (s, p) = line.split_once(": ").unwrap();
            let (r, c) = s.split_once(' ').unwrap();
            let (i, j) = r.split_once('-').unwrap();
            let i = i.parse::<usize>().unwrap();
            let j = j.parse::<usize>().unwrap();

            let c = c.as_bytes()[0];
            let p = p.as_bytes();

            let a = (p[i-1] == c) as u8;
            let b = (p[j-1] == c) as u8;
            (a, b)
        })
        .filter(|(a, b)| (a ^ b) == 1)
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 538);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 489);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 2);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 1);
    }

}