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
    input.lines()
        .map(|s| {
            let bytes = s.as_bytes();
            let mut chars = 0;
            let mut i = 1;
            while i < bytes.len() - 1 {
                i = match (bytes[i] as char, bytes[i+1] as char) {
                    ('\\', '"')  => { chars += 1; i + 2 },
                    ('\\', 'x')  => { chars += 1; i + 4 },
                    ('\\', '\\') => { chars += 1; i + 2 },
                    _            => { chars += 1; i + 1 },
                }
            }

            bytes.len() - chars
        })
        .sum()
}

fn part_two(input: &str) -> usize
{
    input.lines()
        .map(|s| {
            let bytes = s.as_bytes();
            let chars = bytes.iter()
                .flat_map(|&b| 
                    match b as char {
                        '"'  => vec!['\\', '"'],
                        '\\' => vec!['\\', '\\'],
                        c    => vec![c]
                    });

            chars.count() - bytes.len() + 2
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
        assert_eq!(part_one(input), 1342);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 2074);
    }
}