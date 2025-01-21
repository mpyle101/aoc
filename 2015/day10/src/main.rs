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
    let digits = input.as_bytes().iter()
        .map(|b| b - b'0')
        .collect::<Vec<_>>();

    (0..40).fold(digits, |v, _| cycle(&v)).len()
}

fn part_two(input: &str) -> usize
{
    let digits = input.as_bytes().iter()
        .map(|b| b - b'0')
        .collect::<Vec<_>>();

    (0..50).fold(digits, |v, _| cycle(&v)).len()
}

fn cycle(digits: &[u8]) -> Vec<u8>
{
    let mut run = 1;
    let mut curr = digits[0];
    let mut v = digits.iter()
        .skip(1)
        .fold(Vec::new(), |mut v, &d| {
            if d == curr { 
                run += 1;
            } else {
                v.push(run);
                v.push(curr);
                run  = 1;
                curr = d;
            };
            v
        });
    v.push(run);
    v.push(curr);

    v
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 329356);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 4666278);
    }
}