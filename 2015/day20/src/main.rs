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
    let presents = input.parse::<usize>().unwrap();

    let last = presents / 10;
    let mut v = vec![0_usize;last];

    (1..last).for_each(|i| {
        (i..last).step_by(i)
            .for_each(|elf| v[elf] += i * 10);
    });
    
    v.iter().position(|n| *n >= presents).unwrap()
}


fn part_two(input: &str) -> usize
{
    use std::cmp::min;

    let presents = input.parse::<usize>().unwrap();

    let last = presents / 10;
    let mut v = vec![0_usize;last];

    (1..last).for_each(|i| {
        let n = min(i*50, last);
        (i..n).step_by(i)
            .for_each(|elf| v[elf] += i * 11);
    });
    
    v.iter().position(|n| *n >= presents).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 786240);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 831600);
    }
}