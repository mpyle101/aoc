use std::ops::RangeInclusive;

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

struct Part {
    v: u32,
    y: usize,
    x: RangeInclusive<usize>,
}

struct Symbol {
    x: usize,
    y: RangeInclusive<usize>,
}

fn part_one(input: &str) -> u32
{
    use regex::Regex;

    let re = Regex::new(r"([^0-9,\.])|([0-9]+)+").unwrap();

    let mut parts = vec![];
    let mut symbols = vec![];

    input.lines()
        .enumerate()
        .for_each(|(y, line)| {
            re.captures_iter(line)
                .flat_map(|c| c.get(0))
                .map(|m| (m.range(), m.as_str()))
                .for_each(|(x, s)| {
                    let c = s.chars().next().unwrap();
                    if c.is_ascii_digit() {
                        let v = s.parse::<u32>().unwrap();
                        let x_min = if x.start == 0 { 0 } else { x.start - 1 };
                        parts.push(Part { y, v, x: x_min..=x.end });
                    } else {
                        symbols.push(Symbol{ y: y-1..=y+1, x: x.start });
                    }
                });
        });

    parts.iter()
        .map(|part| part_value(part, &symbols))
        .sum()
}

fn part_two(input: &str) -> u32
{
    use regex::Regex;

    let re = Regex::new(r"([^0-9,\.])|([0-9]+)+").unwrap();

    let mut parts = vec![];
    let mut gears = vec![];

    input.lines()
        .enumerate()
        .for_each(|(y, line)| {
            re.captures_iter(line)
                .flat_map(|c| c.get(0))
                .map(|m| (m.range(), m.as_str()))
                .for_each(|(x, s)| {
                    let c = s.chars().next().unwrap();
                    if c.is_ascii_digit() {
                        let v = s.parse::<u32>().unwrap();
                        let x_min = if x.start == 0 { 0 } else { x.start - 1 };
                        parts.push(Part { y, v, x: x_min..=x.end });
                    } else {
                        gears.push(Symbol { y: y-1..=y+1, x: x.start });
                    }
                });
        });

    gears.iter()
        .map(|gear| gear_ratio(gear, &parts))
        .sum()
}

fn part_value(part: &Part, symbols: &[Symbol]) -> u32
{
    if symbols.iter().any(|sym| sym.y.contains(&part.y) && part.x.contains(&sym.x)) {
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
        if gear.y.contains(&p.y) && p.x.contains(&gear.x) {
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
