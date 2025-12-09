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

#[allow(clippy::needless_range_loop)]
fn part_one(input: &str) -> u64
{
    let tiles = input.lines()
        .map(|s| {
            let (s1, s2) = s.split_once(',').unwrap();
            let x = s1.parse::<u64>().unwrap();
            let y = s2.parse::<u64>().unwrap();

            (x, y)
        })
        .collect::<Vec<_>>();

    let mut area = 0;
    for i in 0..tiles.len() - 1 {
        let a = tiles[i];
        for j in i + 1..tiles.len() {
            let b = tiles[j];
            let d = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            area = area.max(d);
        }
    }

    area
}

fn part_two(_input: &str) -> u64
{

    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 4761736832);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 4761736832);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 50);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 50);
    }
}
