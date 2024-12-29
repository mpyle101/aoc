fn main()
{
    use std::time::Instant;

    let input = include_str!("../example.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    let (s1, s2) = input.split_once('\n').unwrap();
    let start = s1.parse::<u32>().unwrap();
    
    s2.split(',')
        .flat_map(|s| s.parse::<u32>())
        .map(|n| (n, n - start % n))
        .min_by(|a, b| a.1.cmp(&b.1))
        .map_or(0, |(n, v)| n * v)
}

fn part_two(input: &str) -> i64
{
    let (_, s2) = input.split_once('\n').unwrap();
    let (r, m): (Vec<_>, Vec<_>) = s2.split(',')
        .zip(0..)
        .flat_map(|(s, i)| s.parse::<i64>().map(|n| (i, n)))
        .unzip();
    let delta = r[r.len() - 1];
    let r = r.iter().map(|n| delta - n).collect::<Vec<_>>();

    crt(&r, &m) - r[0]
}

// Chinese Remainder Theorem
fn crt(residues: &[i64], moduli: &[i64]) -> i64
{
    let prod = moduli.iter().product::<i64>();
    let sum  = residues.iter()
        .zip(moduli)
        .fold(0, |acc, (r, m)| {
            let p = prod / m;
            acc + (r * modinv(p, *m).unwrap() * p)
        });

    sum % prod
}

fn modinv(x: i64, n: i64) -> Option<i64>
{
    use num_integer::Integer;

    let e = x.extended_gcd(&n);
    (e.gcd == 1).then_some((e.x % n + n) % n)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 203);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 905694340256752);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 295);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 1068781);
    }
}