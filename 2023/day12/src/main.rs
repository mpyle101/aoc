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
    input.lines()
        .map(|line| {
            let (springs, s2) = line.split_once(' ').unwrap();
            let groups: Vec<_> = s2.split(',')
                .flat_map(|n| n.parse::<u8>())
                .collect();
            possible(springs.as_bytes(), &groups)
        })
        .sum()
}

fn part_two(input: &str) -> u32
{
    input.lines()
        .map(|line| {
            let (springs, s2) = line.split_once(' ').unwrap();
            let groups: Vec<_> = s2.split(',')
                .flat_map(|n| n.parse::<u8>())
                .collect();
            let mut s = String::from(springs);
            let mut v = groups.clone();
            for _ in 0..4 {
                s = s + &String::from(springs);
                v.extend(groups.clone());
            }

            0
        })
        .sum()
}

fn possible(springs: &[u8], groups: &[u8]) -> u32
{
    arrangements(springs).iter()
        .filter(|v| {
            let mut blocks = vec![];
            let mut count = 0u8;
            for byte in *v {
                if *byte == b'.' {
                    if count > 0 {
                        blocks.push(count);
                        count = 0;
                    }
                } else {
                    count += 1
                }
            }
            if count > 0 {
                blocks.push(count);
            }
            blocks == groups
        })
        .count() as u32
}

fn arrangements(springs: &[u8]) -> Vec<Vec<u8>>
{
    use std::collections::VecDeque;

   if  let Some(i) = springs.iter().position(|c| *c == b'?') {
        let mut v = Vec::new();

        let mut good = Vec::from(springs);
        good[i] = b'.';
        let mut bad = Vec::from(springs);
        bad[i] = b'#';

        let mut working = VecDeque::from([good, bad]);
        while let Some(mut s) = working.pop_front() {
            if let Some(i) = s.iter().position(|c| *c == b'?') {
                let mut good = s.clone();
                good[i] = b'.';
                s[i] = b'#';
                working.push_back(good);
                working.push_back(s);
            } else {
                v.push(s)
            }
        }

        v
    } else {
        vec![springs.into()]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 7307);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 21);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 525152);
    }
}
