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
    let mut v = [0; 8];
    input.lines()
        .filter_map(|line| {
            let len = line.split(' ')
                .filter_map(|s| s.parse().ok())
                .fold(0, |i, n| { v[i] = n; i + 1 });
            is_safe(&v, len).then_some(0)
        })
        .count()
}

fn part_two(input: &str) -> usize
{
    let mut v = [0;8];
    let mut w = [0;7];
    input.lines()
        .filter_map(|line| {
            let len = line.split(' ')
                .filter_map(|s| s.parse().ok())
                .fold(0, |i, n| { v[i] = n; i + 1 });
            (is_safe(&v, len) || (0..len).any(|i| {
                let mut ix = 0;
                (0..i).for_each(|x| { w[ix] = v[x]; ix += 1; });
                (i+1..len).for_each(|x| { w[ix] = v[x]; ix += 1; });
                is_safe(&w, len-1)
            })).then_some(0)
        })
        .count()
}

fn is_safe(v: &[i32], len: usize) -> bool
{
    let dir = (v[1] - v[0]).signum();
    (1..len).all(|i| {
        let d = v[i] - v[i-1];
        d.signum() == dir && (1..4).contains(&d.abs())
    })
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
