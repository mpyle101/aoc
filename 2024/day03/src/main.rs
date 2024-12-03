fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1:  {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_one_a(input);
    println!("Part 1a: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2:  {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> i32
{
    use std::str::FromStr;
    use regex::Regex;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    input.lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|c| {
                   let v1 = i32::from_str(&c[1]).unwrap();
                   let v2 = i32::from_str(&c[2]).unwrap();
                   v1 * v2
                })
                .sum::<i32>()
        })
        .sum()
}

fn part_one_a(input: &str) -> i32
{
    use std::iter::from_fn;

    input.lines()
        .map(|line| {
            let mut i = 0;
            let mut sum = 0;
            while let Some(ix) = line[i..].find("mul(") {
                i += ix + 4;
                let mut chars = line[i..].chars().peekable();
                let s1: String = from_fn(|| chars.next_if(|c| c.is_ascii_digit())).collect();
                i += s1.len();
                if !s1.is_empty() && chars.peek() == Some(&',') {
                    chars.next();
                    i += 1;

                    let s2: String = from_fn(|| chars.next_if(|c| c.is_ascii_digit())).collect();
                    i += s2.len();
                    if !s2.is_empty() && chars.peek() == Some(&')') {
                        chars.next();
                        i += 1;
                        let v1 = s1.parse::<i32>().unwrap();
                        let v2 = s2.parse::<i32>().unwrap();
                        sum += v1 * v2;
                    }
                }
            }
            sum
        })
        .sum()
}

fn part_two(input: &str) -> i32
{
    use std::str::FromStr;
    use regex::Regex;

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    input.lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|c|
                    match &c[0] {
                        "do()"       => { enabled = true; 0 },
                        "don't()"    => { enabled = false; 0 },
                        _ if enabled => {
                                let v1 = i32::from_str(&c[1]).unwrap();
                                let v2 = i32::from_str(&c[2]).unwrap();
                                v1 * v2
                            },
                        _ => 0
                    }
                )
                .sum::<i32>()
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 182780583);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 90772405);
    }

    #[test]
    fn example_part_one()
    {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_one(input), 161);
    }

    #[test]
    fn example_part_two()
    {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_two(input), 48);
    }
}
