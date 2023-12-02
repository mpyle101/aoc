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
            let s = if gid < 10 {
                &line[8..]
            } else if gid == 100 {
                &line[10..]
            } else {
                &line[9..]
            };
            let mut cubes = (0, 0, 0);
            let mut iter = s.split(' ');
            while let Some(count) = iter.next() {
                let v: u32 = count.parse().unwrap();
                let color = iter.next().unwrap();
                let ch = color.chars().next().unwrap();
                match ch {
                    'r' => if cubes.0 < v { cubes.0 = v },
                    'b' => if cubes.1 < v { cubes.1 = v },
                    'g' => if cubes.2 < v { cubes.2 = v },
                     _  => panic!("Unknown color: {ch}")
                }
            }
            (gid, cubes)
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
            let s = if gid < 10 {
                &line[8..]
            } else if gid == 100 {
                &line[10..]
            } else {
                &line[9..]
            };
            let mut cubes = (0, 0, 0);
            let mut iter = s.split(' ');
            while let Some(count) = iter.next() {
                let v: u32 = count.parse().unwrap();
                let color = iter.next().unwrap();
                let ch = color.chars().next().unwrap();
                match ch {
                    'r' => if cubes.0 < v { cubes.0 = v },
                    'b' => if cubes.1 < v { cubes.1 = v },
                    'g' => if cubes.2 < v { cubes.2 = v },
                     _  => panic!("Unknown color: {ch}")
                }
            }
            cubes
        })
        .map(|(r, b, g)| r * b * g)
        .sum()
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
