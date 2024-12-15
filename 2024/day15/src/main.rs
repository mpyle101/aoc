struct Warehouse {
    ncols: usize,
    contents: Vec<char>,
}

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
    let (mut robot, mut wh, moves) = load(input);
    moves.chars()
        .for_each(|c| robot = do_move(c, robot, &mut wh));

    wh.contents.iter()
        .zip(0..)
        .filter(|(&c, _)| c == 'O')
        .map(|(_, i)| 100 * (i / wh.ncols) + (i % wh.ncols))
        .sum()
}

fn part_two(input: &str) -> usize
{
    let (mut robot, mut wh, moves) = load_wide(input);

    moves.chars()
        .for_each(|c| robot = do_move_wide(c, robot, &mut wh));

    wh.contents.iter()
        .zip(0..)
        .filter(|(&c, _)| c == '[')
        .map(|(_, i)| 100 * (i / wh.ncols) + (i % wh.ncols))
        .sum()
}

fn do_move(c: char, robot: usize, wh: &mut Warehouse) -> usize
{
    let offset = match c {
        '^' => -(wh.ncols as i32),
        'v' => wh.ncols as i32,
        '<' => -1,
        '>' => 1,
         _  => unreachable!()
    };

    let mut p = (robot as i32 + offset) as usize;
    if wh.contents[p] == '.' {
        wh.contents[p] = '@';
        wh.contents[robot] = '.';
        p
    } else if wh.contents[p] == '#' {
        robot
    } else {
        while wh.contents[p] == 'O' { p = (p as i32 + offset) as usize }
        if wh.contents[p] == '#' {
            robot
        } else {
            while wh.contents[p] != '@' {
                wh.contents[p] = 'O';
                p = (p as i32 - offset) as usize;
            }
            wh.contents[p] = '.';
            wh.contents[(p as i32 + offset) as usize] = '@';
            (p as i32 + offset) as usize
        }
    }
}

fn do_move_wide(c: char, robot: usize, wh: &mut Warehouse) -> usize
{
    use std::collections::BTreeSet;

    let offset = match c {
        '>' => 1,
        '<' => -1,
        'v' => wh.ncols as i32,
        '^' => -(wh.ncols as i32),
         _  => unreachable!()
    };

    let mut p = (robot as i32 + offset) as usize;
    if wh.contents[p] == '.' {
        wh.contents[p] = '@';
        wh.contents[robot] = '.';
        p
    } else if wh.contents[p] == '#' {
        robot
    } else if c == '<' || c == '>' {
        while wh.contents[p] != '.' && wh.contents[p] != '#' { 
            p = (p as i32 + offset) as usize
        }
        if wh.contents[p] == '#' {
            robot
        } else {
            while wh.contents[p] != '@' {
                wh.contents[p] = wh.contents[(p as i32 - offset) as usize];
                p = (p as i32 - offset) as usize;
            }
            wh.contents[p] = '.';
            wh.contents[(p as i32 + offset) as usize] = '@';
            (p as i32 + offset) as usize
        }
    } else {
        // Collect the set of positions marking halfs of boxes to be moved.
        let mut boxes = if wh.contents[p] == '[' {
            BTreeSet::from([p, p + 1])
        } else {
            BTreeSet::from([p, p - 1])
        };

        let mut blocked = false;
        loop {
            let b = boxes.iter()
                .fold(BTreeSet::new(), |mut v, &p| {
                    let q = (p as i32 + offset) as usize;
                    match wh.contents[q] {
                        '[' => { v.insert(p); v.insert(q); v.insert(q + 1); },
                        ']' => { v.insert(p); v.insert(q); v.insert(q - 1); },
                        '.' => { v.insert(p); },
                         _  => blocked = true,
                    };
                    v
                });
            if blocked || b == boxes { break };
            boxes = b;
        }

        if blocked {
            // We ran into a wall some where so we can't move so just
            // return the current robot position.
            robot
        } else {
            // we need to iterate from the positions we're moving boxes
            // into back to the robot. So, we iterator over the positions
            // based on the movement direction. BTreeSet returns elements
            // in ascending order so reverse if robot is moving them down
            // (offset is positive).
            let move_box = |&b| {
                let q = (b as i32 + offset) as usize;
                wh.contents[q] = wh.contents[b];
                wh.contents[b] = '.';
            };
            if offset < 0 {
                boxes.iter().for_each(move_box);
            } else {
                boxes.iter().rev().for_each(move_box);
            }
            wh.contents[p] = '@';
            wh.contents[robot] = '.';
            p
        }
    }
}

fn load(input: &str) -> (usize, Warehouse, String)
{
    let mut nrows = 0;
    let mut ncols = 0;
    let mut robot = 0;

    let (s1, s2) = input.split_once("\n\n").unwrap();
    let contents = s1.lines()
        .enumerate()
        .fold(vec![], |mut v, (row, line)| {
            nrows = row + 1;
            ncols = line.len();
            if let Some(i) = line.chars().position(|c| c == '@') {
                robot = row * ncols + i;
            }
            v.extend(line.chars());
            v
        });

    let steps = s2.lines().collect::<Vec<_>>().join("");

    (robot, Warehouse { ncols, contents }, steps)
}

fn load_wide(input: &str) -> (usize, Warehouse, String)
{
    let mut nrows = 0;
    let mut ncols = 0;
    let mut robot = 0;

    let (s1, s2) = input.split_once("\n\n").unwrap();
    let contents = s1.lines()
        .enumerate()
        .fold(vec![], |mut v, (row, line)| {
            nrows = row + 1;
            let l: String = line.chars()
                .map(|c| 
                    match c {
                        '#' => "##",
                        'O' => "[]",
                        '.' => "..",
                        '@' => "@.",
                         _  => unreachable!()
                    })
                .collect();
            ncols = l.len();
            if let Some(i) = l.chars().position(|c| c == '@') {
                robot = row * ncols + i;
            }
            v.extend(l.chars());
            v
        });

    let steps = s2.lines().collect::<Vec<_>>().join("");

    (robot, Warehouse { ncols, contents }, steps)
}

#[allow(dead_code)]
fn print(wh: &Warehouse)
{
    wh.contents.iter()
        .zip(0..)
        .for_each(|(c, i)| {
            if i != 0 && i % wh.ncols == 0 { println!() }
            print!("{c}")
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1526018);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1550677);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 2028);

        let input = include_str!("../example2.txt");
        assert_eq!(part_one(input), 10092);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example2.txt");
        assert_eq!(part_two(input), 9021);
    }
}
