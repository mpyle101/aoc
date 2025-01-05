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
    let mut elves = input.parse::<i32>().unwrap();
    let mut elf = 1;

    elves /= 2;
    let mut offset = 2;
    while elves > 1 {
        offset *= 2;
        if elves % 2 == 1 {
            elf += offset;
        }
        elves /= 2;
    }

    elf
}

fn part_two(input: &str) -> usize {
    let elves = input.parse::<usize>().unwrap();

    // Needed to look for the pattern for the first 100 inputs or so.
    let mut elf = 1;

    while elf * 3 < elves {
        elf *= 3
    }
    
    elves - elf
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1830117);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1417887);
    }
}