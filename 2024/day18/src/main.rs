use std::collections::HashSet;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input, 70, 1024);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input, 70);
    println!("Part 2: {:?} ({:?})", result, t.elapsed());
}

fn part_one(input: &str, d: u32, b: usize) -> u32
{
    use pathfinding::prelude::dijkstra;

    let bytes = input.lines()
        .take(b)
        .flat_map(|line| line.split_once(','))
        .map(|(x, y)| {
            let x = x.parse::<u32>().unwrap();
            let y = y.parse::<u32>().unwrap();
            (x, y)
        })
        .collect::<HashSet<_>>();

    let (_, cost) = dijkstra(
        &(0, 0),
        |&p| do_moves(p, d, &bytes),
        |&p| p == (d, d)
    ).unwrap();

    cost
}

fn part_two(input: &str, d: u32) -> (u32, u32)
{
    use pathfinding::prelude::dijkstra;

    let bytes = input.lines()
        .flat_map(|line| line.split_once(','))
        .map(|(x, y)| {
            let x = x.parse::<u32>().unwrap();
            let y = y.parse::<u32>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    let mut fallen = HashSet::new();
    for byte in bytes {
        fallen.insert(byte);
        if dijkstra(&(0, 0), |&p| do_moves(p, d, &fallen), |&p| p == (d, d)).is_none() {
            return byte
        }
    }

    (0, 0)
}

fn do_moves((x, y): (u32, u32), d: u32, bytes: &HashSet<(u32, u32)>) -> Vec<((u32, u32), u32)>
{
    let mut v = Vec::with_capacity(4);

    if x > 0 && !bytes.contains(&(x - 1, y)) { v.push(((x - 1, y), 1)) }
    if y > 0 && !bytes.contains(&(x, y - 1)) { v.push(((x, y - 1), 1)) }
    if x < d && !bytes.contains(&(x + 1, y)) { v.push(((x + 1, y), 1)) }
    if y < d && !bytes.contains(&(x, y + 1)) { v.push(((x, y + 1), 1)) }

    v
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input, 70, 1024), 280);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input, 70), (28, 56));
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input, 6, 12), 22);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input, 6), (6, 1));
    }
}
