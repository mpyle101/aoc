fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input, 80);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str, days: usize) -> u64
{
    // Create a vector of the number of fish at each age.
    let fish = input.split(',')
        .flat_map(|s| s.parse::<usize>())
        .fold([0;9], |mut v, i| { v[i] += 1; v });

    // Rotating the vector to the left one moves the internal
    // timer of each set of fish. The fish at position 8 are
    // all the new fish spawned from the fish at time 0. Those
    // fish then go to time 6.
    (0..days)
        .fold(fish, |mut v, _| {
            v.rotate_left(1);
            v[6] += v[8];
            v
        })
        .iter()
        .sum()
}

fn part_two(input: &str) -> u64
{
    part_one(input, 256)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input, 80), 350917);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1592918715629);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input, 18), 26);
        assert_eq!(part_one(input, 80), 5934);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 26984457539);
    }
}