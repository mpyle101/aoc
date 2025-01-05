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

fn part_one(input: &str) -> String {
    calc_checksum(input, 272)
}

fn part_two(input: &str) -> String {
    calc_checksum(input, 35651584)
}

fn calc_checksum(state: &str, len: usize) -> String {
    let mut data = state.chars().map(|c| c == '1').collect::<Vec<_>>();

    while data.len() <= len {
        let b = data.iter().rev().map(|v| !v).collect::<Vec<_>>();
        data.push(false);
        data.extend(b);
    }

    let mut cs = data[0..len].to_vec();
    while cs.len() % 2 == 0 {
        cs = checksum(&cs);
    }

    cs.iter().map(|v| if *v { '1' } else { '0' }).collect()
}

fn checksum(v: &[bool]) -> Vec<bool> {
    (0..v.len()).step_by(2).map(|i| v[i] == v[i+1]).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), "11100111011101111");
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), "10001110010000110");
    }
}