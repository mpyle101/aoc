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
    use std::str::FromStr;

    input.lines()
        .filter_map(|line| {
            let v = line.split(' ')
                .filter_map(|s| i32::from_str(s).ok())
                .collect::<Vec<_>>();
            is_safe(&v).then_some(0)
        })
        .count()
}

fn part_two(input: &str) -> usize
{
    use std::str::FromStr;

    input.lines()
        .filter_map(|line| {
            let v = line.split(' ')
                .filter_map(|s| i32::from_str(s).ok())
                .collect::<Vec<_>>();
            (is_safe(&v) || (0..v.len()).any(|i| {
                let mut v1 = v.clone();
                v1.remove(i);
                is_safe(&v1)
            })).then_some(0)
        })
        .count()
}

fn is_safe(v: &[i32]) -> bool
{
    let delta = sign(v[1] - v[0]);
    v.windows(2)
        .all(|w| { 
            let d = w[1] - w[0];
            sign(d) == delta && d.abs() > 0 && d.abs() < 4
        })
}

fn sign(n: i32) -> i8 {
    match n {
        n if n > 0 => 1,
        0 => 0,
        _ => -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 218);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 290);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 2);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 4);
    }
}
