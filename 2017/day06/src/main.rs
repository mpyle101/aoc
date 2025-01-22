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

fn part_one(input: &str) -> i32
{
    use std::collections::HashSet;

    let mut banks  = load(input);
    let mut states = HashSet::new();
    while states.insert(banks.clone()) {
        cycle(&mut banks);
    }

    states.len() as i32
}

fn part_two(input: &str) -> i32
{
    use std::collections::HashSet;

    let mut banks  = load(input);
    let mut states = HashSet::new();
    while states.insert(banks.clone()) {
        cycle(&mut banks);
    }

    let state = banks.clone();
    let mut cycles = 1;
    cycle(&mut banks);
    while banks != state { 
        cycle(&mut banks);
        cycles += 1;
    }

    cycles
}

fn load(input: &str) -> Vec<i32>
{
    input.split_whitespace()
        .flat_map(|s| s.parse::<i32>())
        .collect()
}

fn cycle(banks: &mut [i32])
{
    let mut n = *banks.iter().max().unwrap();
    let mut i = banks.iter().position(|v| *v == n).unwrap();

    banks[i] = 0;
    while n > 0 {
        i = (i + 1) % banks.len();
        banks[i] += 1;
        n -= 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 11137);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1037);
    }
}