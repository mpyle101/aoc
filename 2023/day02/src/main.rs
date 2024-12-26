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

fn part_one(input: &str) -> u32
{
    // [r, b, g]
    let max = [13, 15, 14];

    input.lines()
        .zip(1..)
        .flat_map(|(line, gid)|
            line.split_once(':')
                .map(|(_, s)| (gid, cubes(s.trim())))
        )
        .filter(|(_, arr)| (0..3).all(|i| arr[i] < max[i]))
        .map(|(gid, _)| gid as u32)
        .sum()
}

fn part_two(input: &str) -> u32
{
    input.lines()
        .flat_map(|line|
            line.split_once(':')
                .map(|(_, s)| cubes(s.trim()))
        )
        .map(|cubes| cubes.iter().product::<u32>())
        .sum()
}

fn cubes(line: &str) -> [u32;3]
{
    use std::cmp::max;

    let mut cubes = [0, 0, 0];
    let mut iter = line.split(' ');
    while let Some(count) = iter.next() {
        let v: u32 = count.parse().unwrap();
        let color = iter.next().unwrap();
        let c = color.as_bytes()[0];
        let i = (c == b'b') as usize + 2 * (c == b'g') as usize;
        cubes[i] = max(cubes[i], v);
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
