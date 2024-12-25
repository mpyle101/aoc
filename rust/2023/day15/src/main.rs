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

fn part_one(input: &str) -> u32
{
    input.split(',')
        .map(|s| hash(s.as_bytes()))
        .sum()
}

fn part_two(input: &str) -> u32
{
    let mut boxes: Vec<Vec<(&[u8], u8)>> = vec![Vec::new(); 256];

    input.split(',')
        .for_each(|s| {
            let b = s.as_bytes();
            let i = b.iter().position(|c| *c == b'-' || *c == b'=').unwrap();
            let label = &b[0..i];
            let slot = hash(label) as usize;

            if b[i] == b'-' {
                if let Some(n) = boxes[slot].iter().position(|item| item.0 == label) {
                    boxes[slot].remove(n);
                }
            } else if let Some(n) = boxes[slot].iter().position(|item| item.0 == label) {
                boxes[slot][n] = (label, b[i+1] - b'0');
            } else {
                boxes[slot].push((label, b[i+1] - b'0'));
            }
        });

    boxes.iter()
        .zip(1u32..)
        .map(|(v, i)| {
            v.iter()
                .zip(1u32..)
                .map(|((_, p,), j)| i * *p as u32 * j)
                .sum::<u32>()
        })
        .sum()
}

fn hash(s: &[u8]) -> u32
{
    s.iter()
        .fold(0, |mut acc, c| {
            acc += *c as u32;
            acc * 17 % 256
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 512283);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 215827);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 1320);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 145);
    }
}
