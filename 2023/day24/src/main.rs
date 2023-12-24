fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    let stones = load(input);

    collisions(&stones, 200000000000000.0, 400000000000000.0)
}

fn load(input: &str) -> Vec<Stone>
{
    input.lines()
        .map(|line| {
            let (sp, sv) = line.split_once(" @ ").unwrap();

            let mut iter = sp.split(',');
            let x = iter.next().unwrap().trim().parse::<i64>().unwrap();
            let y = iter.next().unwrap().trim().parse::<i64>().unwrap();
            let z = iter.next().unwrap().trim().parse::<i64>().unwrap();

            let mut iter = sv.split(',');
            let xv = iter.next().unwrap().trim().parse::<i64>().unwrap();
            let yv = iter.next().unwrap().trim().parse::<i64>().unwrap();
            let zv = iter.next().unwrap().trim().parse::<i64>().unwrap();

            Stone { p: [x, y, z], v: [xv, yv, zv] }
            
        })
        .collect()
}

#[derive(Debug)]
struct Stone {
    p: [i64;3],
    v: [i64;3],
}

fn collisions(stones: &[Stone], min: f32, max: f32) -> u32
{
    let r = min..=max;
    let is_valid = |p: &(f32, f32)| r.contains(&p.0) && r.contains(&p.1);

    stones.iter()
        .enumerate()
        .map(|(i, s1)| stones.iter()
            .skip(i + 1)
            .filter_map(|s2| intersection_2d(s1, s2))
            .filter(is_valid)
            .count()
        )
        .sum::<usize>() as u32
}

fn intersection_2d(s1: &Stone, s2: &Stone) -> Option<(f32, f32)>
{
    let dx = s2.p[0] - s1.p[0];
    let dy = s2.p[1] - s1.p[1];
    let dt = s2.v[0] * s1.v[1] - s2.v[1] * s1.v[0];
    if dt == 0 {
        return None
    }

    let u = (dy * s2.v[0] - dx * s2.v[1]) as f32 / dt as f32;
    let v = (dy * s1.v[0] - dx * s1.v[1]) as f32 / dt as f32;
    if u < 0.0 || v < 0.0 {
        return None
    }

    Some((
        s1.p[0] as f32 + s1.v[0] as f32 * u,
        s1.p[1] as f32 + s1.v[1] as f32 * u)
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 16050);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        let stones = load(input);
        assert_eq!(collisions(&stones, 7.0, 27.0), 2);
    }
}
