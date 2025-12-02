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

fn part_one(input: &str) -> u64
{
    input.split(',')
        .fold(0, |acc, rng| {
            let (s1, s2) = rng.split_once('-').unwrap();
            let n1 = s1.parse::<u64>().unwrap();
            let n2 = s2.parse::<u64>().unwrap();

            acc + (n1..=n2).filter(|n| is_repeated(*n)).sum::<u64>()
        })
}

fn part_two(input: &str) -> u64
{
    input.split(',')
        .fold(0, |acc, rng| {
            let (s1, s2) = rng.split_once('-').unwrap();
            let n1 = s1.parse::<u64>().unwrap();
            let n2 = s2.parse::<u64>().unwrap();

            acc + (n1..=n2).filter(|n| is_repeated_n(*n)).sum::<u64>()
        })
}

fn is_repeated(mut n: u64) -> bool
{
    let mut digits = [0u8;10];

    let mut i = 0;
    while n > 0 {
        digits[i] = (n % 10) as u8;
        n /= 10;
        i += 1;
    }

    if i.is_multiple_of(2) {
        let k = i / 2;
        digits[0..k] == digits[k..k + k]
    } else {
        false
    }
}

fn is_repeated_n(mut n: u64) -> bool
{
    let mut digits = vec![];

    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits.reverse();

    for n in 1..=digits.len() / 2 {
        let mut iter = digits.chunks(n);
        let c1 = iter.next().unwrap();
        if iter.all(|c2| c2 == c1) { return true }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 31000881061);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 46769308485);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 1227775554);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 4174379265);
    }
}
