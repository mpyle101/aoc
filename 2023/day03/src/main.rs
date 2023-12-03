use std::collections::HashSet;

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
    let mut symbols = HashSet::new();

    input.lines()
        .enumerate()
        .for_each(|(x, line)| {
            let mut v = 0;
            let mut idx = 0;
            let mut pos = [(-1, -1); 3];
            let mut last = b'.';

            line.as_bytes().iter()
                .enumerate()
                .for_each(|(y, c)| {
                    if c.is_ascii_digit() {
                        v = v * 10 + (*c - b'0') as u32;
                        pos[idx] = (x as i32, y as i32);
                        idx += 1;
                    } else {
                        if last.is_ascii_digit() {
                            parts.push((v, pos));
                            v = 0;
                            idx = 0;
                            pos = [(-1, -1); 3];
                        }
                        if *c != b'.' {
                            let r = x as i32;
                            let c = y as i32;
                            symbols.extend([
                                (r-1, c-1), (r-1, c), (r-1, c+1), (r, c-1),
                                (r, c+1), (r+1, c-1), (r+1, c), (r+1, c+1)
                            ]);
                        }
                    }
                    last = *c;
                });

            if last.is_ascii_digit() {
                parts.push((v, pos));
            }
        });

    parts.iter()
        .filter_map(|(v, pos)| 
            pos.iter().any(|p| symbols.contains(p)).then_some(v)
        )
        .sum()
}

fn part_two(input: &str) -> u32
{
    let mut parts = vec![];
    let mut gears = vec![];

    input.lines()
        .enumerate()
        .for_each(|(x, line)| {
            let mut v = 0;
            let mut idx = 0;
            let mut pos = [(-1, -1); 3];
            let mut last = b'.';
            
            line.as_bytes().iter()
                .enumerate()
                .for_each(|(y, c)| {
                    if c.is_ascii_digit() {
                        v = v * 10 + (*c - b'0') as u32;
                        pos[idx] = (x as i32, y as i32);
                        idx += 1;
                    } else {
                        if last.is_ascii_digit() {
                            parts.push((v, pos));
                            
                            v = 0;
                            idx = 0;
                            pos = [(-1, -1); 3];
                        }
                        if *c == b'*' {
                            gears.push((x as i32, y as i32));
                        }
                    }
                    last = *c;
                });

            if last.is_ascii_digit() {
                parts.push((v, pos));
            }
        });

    gears.iter()
        .map(|gear| gear_ratio(gear, &parts))
        .sum()
}

fn gear_ratio((x, y): &(i32, i32), parts: &[(u32, [(i32, i32); 3])]) -> u32
{
    let aura = [
        (x-1, y-1), (x-1, *y), (x-1, y+1), (*x, y-1),
        (*x, y+1), (x+1, y-1), (x+1, *y), (x+1, y+1)
    ];

    let mut pn = [0, 0];
    let mut idx = 0;

    for (v, pos) in parts {
        if pos.iter().any(|p| *p != (-1, -1) && aura.contains(p)) {
            if idx == 2 { return 0; } // too many parts

            pn[idx] = *v;
            idx += 1;
        }
    }

    if pn[0] == 0 { 0 } else { pn[0] * pn[1] }
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
