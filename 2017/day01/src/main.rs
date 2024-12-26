fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} {:?}", result, t.elapsed());
}

fn part_one(input: &str) -> i32
{
    let captcha: i32 = input.as_bytes()
        .windows(2)
        .filter(|w| w[0] == w[1])
        .map(|w| (w[1] - b'0') as i32)
        .sum();

    captcha + if input.chars().last() == input.chars().next() {
        (input.bytes().last().unwrap() - b'0') as i32
    } else {
        0
    }
}

fn part_two(input: &str) -> i32
{
    let n = input.len() / 2;

    let bytes = input.as_bytes();
    bytes.iter()
        .enumerate()
        .filter(|(i, b)| {
            let ix = (i + n) % input.len();
            bytes[ix] == **b
        })
        .map(|(_, b)| (*b - b'0') as i32)
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1119);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1420);
    }
}