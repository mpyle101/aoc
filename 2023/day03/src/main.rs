use std::collections::HashMap;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let parts = part_one(input);
    println!("Part 1: {} ({:?})", parts, t.elapsed());

    let t = Instant::now();
    let gears = part_two(input);
    println!("Part 2: {} ({:?})", gears, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    let mut parts = vec![];
    let mut symbols = vec![];

    input.lines()
        .enumerate()
        .for_each(|(x, line)| {
            let mut v = 0u32;
            let mut p = (0, 0);
            let mut last = b'.';

            line.as_bytes().iter()
                .enumerate()
                .for_each(|(y, c)| {
                    if c.is_ascii_digit() {
                        if v == 0 { p = (x as i32, y as i32) }
                        v = v * 10 + (*c - b'0') as u32
                    } else {
                        if last.is_ascii_digit() {
                            parts.push((v, p));
                            v = 0;
                            p = (0, 0);
                        }
                        if *c != b'.' {
                            symbols.push((x as i32, y as i32))
                        }
                    }
                    last = *c;
                });

            if last.is_ascii_digit() {
                parts.push((v, p));
            }
        });

    parts.iter()
        .filter(|&&part| is_adjacent(part, &symbols))
        .map(|(v, _)| v)
        .sum()
}

fn part_two(input: &str) -> u32
{
    let mut parts = vec![];
    let mut gears: HashMap<(i32, i32), Vec<u32>> = HashMap::new();

    input.lines()
        .enumerate()
        .for_each(|(x, line)| {
            let mut v = 0;
            let mut p = (0, 0);
            let mut last = b'.';

            line.as_bytes().iter()
                .enumerate()
                .for_each(|(y, c)| {
                    if c.is_ascii_digit() {
                        if v == 0 { p = (x as i32, y as i32) }
                        v = v * 10 + (*c - b'0') as u32
                    } else {
                        if last.is_ascii_digit() {
                            parts.push((v, p));
                            v = 0;
                            p = (0, 0);
                        }
                        if *c == b'*' {
                            gears.insert((x as i32, y as i32), Vec::new());
                        }
                    }
                    last = *c;
                });

            if last.is_ascii_digit() {
                parts.push((v, p));
            }
        });

    parts.iter().for_each(|&part| update_gears(part, &mut gears) );
    gears.values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<u32>())
        .sum()
}

fn is_adjacent(part: (u32, (i32, i32)), symbols: &[(i32, i32)]) -> bool
{
    let pos = positions(part);
    pos.iter().any(|p| symbols.contains(p))
}

fn update_gears(part: (u32, (i32, i32)), gears: &mut HashMap<(i32, i32), Vec<u32>>)
{
    let pos = positions(part);
    for (k, v) in gears {
        if pos.contains(k) {
            v.push(part.0)
        }
    }
}

fn positions((v, (x, y)): (u32, (i32, i32))) -> [(i32, i32); 12]
{
    let mut pos = [
        (x-1, y-1), (x-1, y), (x-1, y+1),(x, y-1),
        (x+1, y-1), (x+1, y), (x+1, y+1),(x, y+1),
        (-1, -1), (-1, -1), (-1, -1), (-1, -1)
    ];
    match v.ilog(10) {
        0 => {},
        1 => {
            pos[7] = (x, y+2);
            pos[8] = (x-1, y+2);
            pos[9] = (x+1, y+2);
        },
        2 => {
            pos[7]  = (x, y+3);
            pos[8]  = (x-1, y+2);
            pos[9]  = (x+1, y+2);
            pos[10] = (x-1, y+3);
            pos[11] = (x+1, y+3);
        },
        _ => panic!("Big number: {v}")
    }

    pos
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 539590);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 80703636);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 4361);
    }
}
