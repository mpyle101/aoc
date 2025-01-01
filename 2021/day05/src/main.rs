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
    use std::collections::HashMap;

    let lines = load(input);

    lines.iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .fold(HashMap::new(), |mut m, &((x1, y1), (x2, y2))| {
            let (dx, dy) = ((x2 - x1).signum(), (y2 -y1).signum());
            let (mut x, mut y) = (x1, y1);
            loop {
                *m.entry((x, y)).or_insert(0) += 1;
                if (x, y) == (x2, y2) { break }
                (x, y) = (x + dx, y + dy);
            }
            m
        })
        .values()
        .filter(|n| **n > 1)
        .count()
}

fn part_two(input: &str) -> usize
{
    use std::collections::HashMap;

    let lines = load(input);

    lines.iter()
        .fold(HashMap::new(), |mut m, &((x1, y1), (x2, y2))| {
            let (dx, dy) = ((x2 - x1).signum(), (y2 -y1).signum());
            let (mut x, mut y) = (x1, y1);
            loop {
                *m.entry((x, y)).or_insert(0) += 1;
                if (x, y) == (x2, y2) { break }
                (x, y) = (x + dx, y + dy);
            }
            m
        })
        .values()
        .filter(|n| **n > 1)
        .count()
}

fn load(input: &str) -> Vec<((i32, i32), (i32, i32))>
{
    input.lines()
        .flat_map(|line| line.split_once(" -> "))
        .map(|(s1, s2)| {
            let (x1, y1) = s1.split_once(',').unwrap();
            let x1 = x1.parse::<i32>().unwrap();
            let y1 = y1.parse::<i32>().unwrap();

            let (x2, y2) = s2.split_once(',').unwrap();
            let x2 = x2.parse::<i32>().unwrap();
            let y2 = y2.parse::<i32>().unwrap();

            ((x1, y1), (x2, y2))
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 7085);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 20271);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 5);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 12);
    }
}