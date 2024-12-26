fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one_parsing(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two_matching(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one_parsing(input: &str) -> u32
{
    use std::iter::from_fn;

    input.lines()
        .fold(0, |mut acc, line| {
            let mut i = 0;
            while let Some(ix) = line[i..].find("mul(") {
                i += ix + 4;
                let mut chars = line[i..].chars().peekable();

                let v1 = from_fn(|| chars.next_if(|c| c.is_ascii_digit()))
                    .fold(0, |acc, c| { i += 1; acc * 10 + c.to_digit(10).unwrap() });
                if v1 > 0 && chars.peek() == Some(&',') {
                    chars.next();
                    i += 1;

                    let v2 = from_fn(|| chars.next_if(|c| c.is_ascii_digit()))
                        .fold(0, |acc, c| { i += 1; acc * 10 + c.to_digit(10).unwrap() });
                    if v2 > 0 && chars.peek() == Some(&')') {
                        chars.next();
                        i += 1;
                        acc += v1 * v2;
                    }
                }
            }
            acc
        })
}

#[allow(dead_code)]
fn part_one_matching(input: &str) -> i32
{
    use std::str::FromStr;
    use regex::Regex;

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    input.lines()
        .map(|line| {
            re.find_iter(line)
                .map(|m| {
                    let (s1, s2) = m.as_str().split_once(',').unwrap();
                    let v1 = i32::from_str(&s1[4..]).unwrap();
                    let v2 = i32::from_str(&s2[..s2.len() - 1]).unwrap();
                    v1 * v2
                })
                .sum::<i32>()
        })
        .sum()
}

#[allow(dead_code)]
fn part_one_capturing(input: &str) -> i32
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

fn part_two_matching(input: &str) -> i32
{
    use std::str::FromStr;
    use regex::Regex;

    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    input.lines()
        .map(|line| {
            re.find_iter(line)
                .map(|m| {
                    match m.as_str() {
                        "do()"       => { enabled = true; 0 },
                        "don't()"    => { enabled = false; 0 },
                        _ if enabled => {
                                let (s1, s2) = m.as_str().split_once(',').unwrap();
                                let v1 = i32::from_str(&s1[4..]).unwrap();
                                let v2 = i32::from_str(&s2[..s2.len() - 1]).unwrap();
                                v1 * v2
                            },
                        _ => 0
                    }
                })
                .sum::<i32>()
        })
        .sum()
}

#[allow(dead_code)]
fn part_two_capturing(input: &str) -> i32
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
    fn input_part_one_parsing()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one_parsing(input), 182780583);
    }
    #[test]
    fn input_part_one_matching()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one_matching(input), 182780583);
    }

    #[test]
    fn input_part_one_capturing()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one_capturing(input), 182780583);
    }

    #[test]
    fn input_part_two_matching()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two_matching(input), 90772405);
    }

    #[test]
    fn input_part_two_capturing()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two_capturing(input), 90772405);
    }

    #[test]
    fn example_part_one_parsing()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one_parsing(input), 161);
    }

    #[test]
    fn example_part_one_matching()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one_matching(input), 161);
    }

    #[test]
    fn example_part_one_capturing()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one_capturing(input), 161);
    }

    #[test]
    fn example_part_two_matching()
    {
        let input = include_str!("../example2.txt");
        assert_eq!(part_two_matching(input), 48);
    }

    #[test]
    fn example_part_two_capturing()
    {
        let input = include_str!("../example2.txt");
        assert_eq!(part_two_capturing(input), 48);
    }
}
