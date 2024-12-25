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
    use std::str::FromStr;

    let [mut v1, mut v2] = input.lines()
        .fold([vec![], vec![]], |mut v, line| {
            line.split_whitespace()
                .enumerate()
                .filter_map(|(i, s)| u32::from_str(s).map(|n| (i, n)).ok())
                .for_each(|(i, n)| v[i].push(n));
            v
        });

    v1.sort_unstable();
    v2.sort_unstable();
    v1.iter()
        .zip(v2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part_two(input: &str) -> u32
{
    use utils::map;
    use std::str::FromStr;

    let [m1, m2] = input.lines()
        .fold([map![], map![]], |mut m, line| {
            line.split_whitespace()
                .enumerate()
                .filter_map(|(i, s)| u32::from_str(s).map(|n| (i, n)).ok())
                .for_each(|(i, n)| *m[i].entry(n).or_insert(0) += 1);
            m
        });

    m1.iter()
        .filter_map(|(k, v)| m2.get(k).map(|n| k * v * n))
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 3714264);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 18805872);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 11);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 31);
    }
}
