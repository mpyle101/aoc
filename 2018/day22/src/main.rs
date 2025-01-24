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

fn part_one(input: &str) -> i32
{
    let (depth, target) = load(input);
    let dims  = (target.0 + 1, target.1 + 1);
    let caves = spelunk(depth, target, dims);
    
    caves.iter().map(|v| v % 3).sum()
}

fn part_two(input: &str) -> i32
{
    use pathfinding::prelude::dijkstra;

    // Pre-calculate extra regions.
    let (depth, target) = load(input);
    let (mx, my) = (target.0 + 30, target.1 + 30);
    let caves = spelunk(depth, target, (mx, my));

    let caves = caves.iter().map(|v| v % 3).collect::<Vec<_>>();

    // nothing, torch, climbing gear => 0, 1, 2
    let start = ((0, 0), 1);    // (0, 0) + torch
    let goal  = (target, 1);    // target + torch
    let result = dijkstra(
        &start,
        |st| successors(st, &caves, mx, my),
        |st| *st == goal
    ).unwrap();

    result.1
}

fn load(input: &str) -> (i32, (i32, i32))
{
    let (s1, s2) = input.split_once('\n').unwrap();

    let (_, s) = s1.split_once(": ").unwrap();
    let depth = s.parse::<i32>().unwrap();

    let (_, s) = s2.split_once(": ").unwrap();
    let (x, y) = s.split_once(',').unwrap();
    let x = x.parse::<i32>().unwrap();
    let y = y.parse::<i32>().unwrap();

    (depth, (x, y))
}

type State = ((i32, i32), i32);

fn successors((p, eq): &State, caves: &[i32], mx: i32, my: i32) -> Vec<(State, i32)>
{
    let (x, y)  = *p;
    let terrain = caves[(y * mx + x) as usize];

    // Start with changing gear costing 7 minutes.
    let mut states = match terrain {
        0 if *eq == 1 => vec![((*p, 2), 7)],
        0 if *eq == 2 => vec![((*p, 1), 7)],
        1 if *eq == 0 => vec![((*p, 2), 7)],
        1 if *eq == 2 => vec![((*p, 0), 7)],
        2 if *eq == 0 => vec![((*p, 1), 7)],
        2 if *eq == 1 => vec![((*p, 0), 7)],
        _ => vec![]
    };

    let mut regions = vec![];
    if x > 0 { regions.push((x-1, y)) }
    if y > 0 { regions.push((x, y-1)) }
    if x < mx-1 { regions.push((x+1, y)) }
    if y < my-1 { regions.push((x, y+1)) }

    regions.iter()
        .filter(|&(x, y)| *eq != caves[(y * mx + x) as usize])
        .for_each(|&pt| states.push(((pt, *eq), 1)));

    states
}

fn spelunk(depth: i32, (tx, ty): (i32, i32), (mx, my): (i32, i32)) -> Vec<i32>
{
    let mut caves = Vec::with_capacity((mx * my) as usize);

    caves.push(depth % 20183);  // (0, 0)
    for x in 1..mx { caves.push((x * 16807 + depth) % 20183) }
    for y in 1..my {
        caves.push((y * 48271 + depth) % 20183);
        for x in 1..mx {
            let geologic_index = if (x, y) == (tx, ty) {
                0
            } else {
                let ex = caves[((y * mx) + x - 1) as usize];
                let ey = caves[((y - 1) * mx + x) as usize];
                ex * ey
            };
            caves.push((geologic_index + depth) % 20183)
        }
    }

    caves
}

#[allow(dead_code)]
fn print(caves: &[i32], (mx, my): (i32, i32))
{
    for y in 0..my {
        for x in 0..mx {
            let terrain = caves[(y * mx + x) as usize];
            match terrain % 3 {
                0 => print!("."),
                1 => print!("="),
                2 => print!("|"),
                _ => unreachable!()
            }
        }
        println!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 11359);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 976);
    }
}