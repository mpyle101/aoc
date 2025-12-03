fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} {:?}", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} {:?}", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    use std::collections::HashSet;

    input.lines()
        .map(|s| {
            let words  = s.split(' ').count();
            let unique = HashSet::<&str>::from_iter(s.split(' '));
            (unique.len() == words) as u32
        })
        .sum()
}

#[allow(clippy::needless_range_loop)]
fn part_two(input: &str) -> u32
{
    input.lines()
        .map(|line| {
            let words = line.split_whitespace()
                .map(|s| {
                    let mut w = s.bytes().collect::<Vec<_>>();
                    w.sort();
                    w
                })
                .collect::<Vec<_>>();

            for i in 0..words.len()-1 {
                let w1 = &words[i];
                for j in i+1..words.len() {
                    let w2 = &words[j];
                    if w1 == w2 { return 0 }
                }
            }

            1
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
        assert_eq!(part_one(input), 325);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 119);
    }
}
