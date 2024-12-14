use std::fs::File;
use std::{collections::HashSet, hash::Hash};

type Position = (i32, i32);
type Velocity = (i32, i32);

#[derive(Eq, Clone, Copy, Debug, Hash, PartialEq)]
struct Robot {
    p: Position,
    v: Velocity,
}

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input, 103, 101);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input, 103, 101);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str, nrows: i32, ncols: i32) -> usize
{
    let mut robots = load(input);
    (0..100).for_each(|_| 
        robots.iter_mut()
            .for_each(|robot| { *robot = move_robot(*robot, nrows, ncols); })
    );
    [
        (0..ncols / 2, 0..nrows / 2),
        (ncols / 2 + 1..ncols, 0..nrows / 2),
        (0..ncols / 2, nrows / 2 + 1..nrows),
        (ncols / 2 + 1..ncols, nrows / 2 + 1..nrows)
    ].iter()
        .map(|(x, y)| robots.iter()
            .filter(|r| x.contains(&r.p.0) && y.contains(&r.p.1))
            .count())
        .product()
}

fn part_two(input: &str, nrows: i32, ncols: i32) -> usize
{
    use std::hash::{DefaultHasher, Hasher};

    let mut robots = load(input);
    let mut hashes = HashSet::new();

    let mut steps = 1;
    'outer: loop {
        robots.iter_mut()
            .for_each(|robot| { *robot = move_robot(*robot, nrows, ncols); });
        let positions = robots.iter().map(|r| r.p).collect::<HashSet<_>>();

        // Look for the top a tree anywhere in the data. We don't need the
        // (0, 0) but it looks nicer. :)
        let tree_top = [
                              (0, 0),
                     (-1, 1), (0, 1), (1, 1),
            (-2, 2), (-1, 2), (0, 2), (1, 2), (2, 2)
        ];
        for p in &positions {
            let found = tree_top.iter()
                .all(|d| {
                    let p = (p.0 + d.0, p.1 + d.1);
                    positions.contains(&p)
                });
            if found { break 'outer }
        }

        // Stop when we start to cycle.
        let mut hasher = DefaultHasher::new();
        robots.hash(&mut hasher);
        let hash = hasher.finish();
        if !hashes.insert(hash) {
            break;
        }
        steps += 1
    };

    steps
}

fn move_robot(robot: Robot, nrows: i32, ncols: i32) -> Robot
{
    let c = robot.p.0 + robot.v.0;
    let x = if c < 0 { 
        ncols + c
    } else if c >= ncols {
        c - ncols
    } else {
        c
    };

    let r = robot.p.1 + robot.v.1;
    let y = if r < 0 {
        nrows + r
    } else if r >= nrows {
        r - nrows
    } else {
        r
    };

    Robot { p: (x, y), ..robot }
}

#[allow(dead_code)]
fn print(robots: &HashSet<Position>)
{
    for y in 0..103 {
        for x in 0..101 {
            if robots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn write(robots: HashSet<Position>, n: i32, file: &mut File)
{
    use std::io::Write;

    let mut s = format!("{n}\n");
    for y in 0..103 {
        for x in 0..101 {
            if robots.contains(&(x, y)) {
                s += "#";
            } else {
                s += ".";
            }
        }
        s += "\n";
    }
    s += "\n";

    file.write_all(s.as_bytes()).unwrap();
}

fn load(input: &str) -> Vec<Robot>
{
    input.lines()
        .map(|line| {
            let (p, v) = line.split_once(' ').unwrap();
            let (sx, sy) = p[2..].split_once(',').unwrap();
            let x = sx.parse::<i32>().unwrap();
            let y = sy.parse::<i32>().unwrap();

            let (sx, sy) = v[2..].split_once(',').unwrap();
            let dx = sx.parse::<i32>().unwrap();
            let dy = sy.parse::<i32>().unwrap();

            Robot { p: (x, y), v: (dx, dy) }
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
        assert_eq!(part_one(input, 103, 101), 230900224);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input, 103, 101), 6532);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input, 7, 11), 12);
    }
}
