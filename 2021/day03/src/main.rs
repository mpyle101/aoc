fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input, 0xFFF);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str, mask: u32) -> u32
{
    let gamma = input.lines()
        .fold([0;12], |mut arr, s| {
            s.chars()
                .rev()
                .enumerate()
                .for_each(|(i, c)| arr[i] += if c == '1' { 1 } else { -1 });
            arr
        })
        .iter()
        .enumerate()
        .filter(|(_, &n)| n > 0)
        .fold(0, |acc, (i, _)| acc | (1 << i));
    let epsilon = !gamma & mask;
    
    gamma * epsilon
}

fn part_two(input: &str) -> u32
{
    let nums = input.lines()
        .map(|s| s.as_bytes())
        .collect::<Vec<_>>();

    gamma(&nums, 0) * epsilon(&nums, 0)
}

fn gamma(nums: &[&[u8]], i: usize) -> u32
{
    if nums.len() == 1 {
        to_num(nums[0])
    } else {
        let (ones, zeros) = nums.iter()
            .fold((vec![], vec!{}), |(mut a, mut b), &v| {
                if v[i] == b'1' { a.push(v) } else { b.push(v) };
                (a, b)
            });

        if ones.len() >= zeros.len() {
            gamma(&ones, i + 1)
        } else {
            gamma(&zeros, i + 1)
        }
    }
}

fn epsilon(nums: &[&[u8]], i: usize) -> u32
{
    if nums.len() == 1 {
        to_num(nums[0])
    } else {
        let (ones, zeros) = nums.iter()
            .fold((vec![], vec!{}), |(mut a, mut b), &v| {
                if v[i] == b'1' { a.push(v) } else { b.push(v) };
                (a, b)
            });

        if zeros.len() <= ones.len() {
            epsilon(&zeros, i + 1)
        } else {
            epsilon(&ones, i + 1)
        }
    }
}

fn to_num(bits: &[u8]) -> u32
{
    bits.iter()
        .rev()
        .enumerate()
        .filter(|(_, &c)| c == b'1')
        .fold(0, |n, (i, _)| n | (1 << i))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input, 0xFFF), 2583164);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 2784375);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input, 0x1F), 198);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 230);
    }

}