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
    input.split("\n\n")
        .map(|passport| {
            passport.lines()
                .flat_map(|line| line.split(' ')
                    .map(|s| &s[0..3])
                    .filter(|s| *s != "cid")
                )
                .collect::<Vec<_>>()
        })
        .filter(|fields| fields.len() == 7)
        .count()
}

static ECL: [&str;7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn part_two(input: &str) -> usize
{
    input.split("\n\n")
        .map(|passport| {
            passport.lines()
                .flat_map(|line| line.split(' '))
                .collect::<Vec<_>>()
        })
        .filter(|fields| fields.len() >= 7)
        .filter(|fields| {
            fields.iter().all(|field|
                match field.split_once(':') {
                    Some(("byr", v)) => in_range(v, 1920..=2002),
                    Some(("iyr", v)) => in_range(v, 2010..=2020),
                    Some(("eyr", v)) => in_range(v, 2020..=2030),
                    Some(("hcl", v)) => v.len() == 7 && u32::from_str_radix(&v[1..], 16).is_ok(),
                    Some(("ecl", v)) => ECL.contains(&v),
                    Some(("pid", v)) => v.len() == 9 && v.parse::<u32>().is_ok(),
                    Some(("hgt", v)) =>
                        if v.ends_with("cm") {
                            in_range(&v[0..v.len()-2], 150..=193)
                        } else {
                            in_range(&v[0..v.len()-2], 59..=76)
                        }
                    Some(("cid", _)) => fields.len() > 7,
                    _ => false
                }
            )
        })
        .count()
}

fn in_range(v: &str, range: std::ops::RangeInclusive<u32>) -> bool
{
    v.parse::<u32>().is_ok_and(|n| range.contains(&n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 196);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 114);
    }
}