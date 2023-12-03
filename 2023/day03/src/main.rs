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

#[derive(Clone, Copy)]
struct Part {
    v: u32,
    x: usize,
    y1: usize,
    y2: usize,
}

#[derive(Clone, Copy)]
struct Symbol {
    t: usize,   // top
    l: usize,   // left
    b: usize,   // bottom
    r: usize,   // right
}

fn part_one(input: &str) -> u32
{
    let mut parts = vec![];
    let mut symbols = vec![];

    input.lines()
        .enumerate()
        .for_each(|(x, line)| {
            let mut last = b'.';
            let mut part = Part { v: 0, x: 0, y1: 0, y2: 0 };

            line.as_bytes().iter()
                .enumerate()
                .for_each(|(y, c)| {
                    if c.is_ascii_digit() {
                        part.x = x;
                        part.y2 = y;
                        if part.v == 0 { part.y1 = y; }
                        part.v = part.v * 10 + (*c - b'0') as u32;
                    } else {
                        if last.is_ascii_digit() {
                            parts.push(part);
                            part = Part { v: 0, x: 0, y1: 0, y2: 0 };
                        }
                        if *c != b'.' {
                            symbols.push(Symbol{ t: x-1, l: y-1, b: x+1, r: y+1 });
                        }
                    }
                    last = *c;
                });

            if last.is_ascii_digit() {
                parts.push(part);
            }
        });

    parts.iter()
        .map(|part| part_value(part, &symbols))
        .sum()
}

fn part_two(input: &str) -> u32
{
    let mut parts = vec![];
    let mut gears = vec![];

    input.lines()
        .enumerate()
        .for_each(|(x, line)| {
            let mut last = b'.';
            let mut part = Part { v: 0, x: 0, y1: 0, y2: 0 };
            
            line.as_bytes().iter()
                .enumerate()
                .for_each(|(y, c)| {
                    if c.is_ascii_digit() {
                        part.x = x;
                        part.y2 = y;
                        if part.v == 0 { part.y1 = y; }
                        part.v = part.v * 10 + (*c - b'0') as u32;
                    } else {
                        if last.is_ascii_digit() {
                            parts.push(part);
                            part = Part { v: 0, x: 0, y1: 0, y2: 0 };
                        }
                        if *c == b'*' {
                            gears.push(Symbol{ t: x-1, l: y-1, b: x+1, r: y+1 });
                        }
                    }
                    last = *c;
                });

            if last.is_ascii_digit() {
                parts.push(part);
            }
        });

    gears.iter()
        .map(|gear| gear_ratio(gear, &parts))
        .sum()
}

fn part_value(part: &Part, symbols: &[Symbol]) -> u32
{
    if symbols.iter().any(|sym| 
        part.x >= sym.t && part.x <= sym.b && part.y2 >= sym.l && part.y1 <= sym.r
    ) {
        part.v
    } else {
        0
    }
}

fn gear_ratio(gear: &Symbol, parts: &[Part]) -> u32
{
    let mut pn = [0, 0];
    let mut idx = 0;

    for p in parts {
        if p.x >= gear.t && p.x <= gear.b && p.y2 >= gear.l && p.y1 <= gear.r {
            if idx == 2 { return 0; } // too many parts

            pn[idx] = p.v;
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
