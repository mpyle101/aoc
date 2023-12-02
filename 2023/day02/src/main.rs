fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let games = part_one(input);
    println!("Part 1: {} ({:?})", games, t.elapsed());

    let t = Instant::now();
    let games = part_two(input);
    println!("Part 2: {} ({:?})", games, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    input.lines()
        .enumerate()
        .map(|(id, line)| {
            let gid = id + 1;
            let idx = (8 + gid.ilog(10)) as usize;
            (gid, cubes(&line[idx..]))
        })
        .filter(|(_, (r, b, g))| *r <= 12 && *b <= 14 && *g <= 13)
        .map(|(gid, _)| gid)
        .sum()
}

fn part_two(input: &str) -> u32
{
    input.lines()
        .enumerate()
        .map(|(id, line)| {
            let gid = id + 1;
            let idx = (8 + gid.ilog(10)) as usize;
            cubes(&line[idx..])
        })
        .map(|(r, b, g)| r * b * g)
        .sum()
}

fn cubes(line: &str) -> (u32, u32, u32)
{
    use std::cmp::max;

    let mut cubes = (0, 0, 0);
    let mut iter = line.split(' ');
    while let Some(count) = iter.next() {
        let v: u32 = count.parse().unwrap();
        let color = iter.next().unwrap();
        let ch = color.as_bytes()[0];
        match ch {
            b'r' => cubes.0 = max(cubes.0, v),
            b'b' => cubes.1 = max(cubes.1, v),
            b'g' => cubes.2 = max(cubes.2, v),
              _  => panic!("Unknown color: {ch}")
        }
    }

    cubes
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1867);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 84538);
    }
}
