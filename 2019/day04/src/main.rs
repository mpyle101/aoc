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
    let (s1, s2) = input.split_once('-').unwrap();
    let n1 = s1.parse::<u32>().unwrap();
    let n2 = s2.parse::<u32>().unwrap();

    (n1..n2)
        .map(digits)
        .filter(|d| d.windows(2).all(|w| w[0] <= w[1]))
        .filter(|d| d.windows(2).any(|w| w[0] == w[1]))
        .count()
}

fn part_two(input: &str) -> usize
{
    let (s1, s2) = input.split_once('-').unwrap();
    let n1 = s1.parse::<u32>().unwrap();
    let n2 = s2.parse::<u32>().unwrap();

    (n1..n2)
        .map(digits)
        .filter(repeat)
        .filter(|d| d.windows(2).all(|w| w[0] <= w[1]))
        .count()
}

fn digits(mut val: u32) -> [u32;6]
{
    let mut i = 5i32;
    let mut arr = [0;6];
    while val > 0 {
        arr[i as usize] = val % 10;
        val /= 10;
        i -= 1
    }

    arr
}

fn repeat(digits: &[u32;6]) -> bool
{
    let mut count = 1;
    for w in digits.windows(2) {
        if w[1] != w[0] {
            if count == 2 {
                return true
            }
            count = 1;
        } else {
            count += 1
        }
    }

    count == 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 2814);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1991);
    }
}