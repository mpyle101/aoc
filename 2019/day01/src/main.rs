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
    input.lines()
        .flat_map(|s| s.parse::<i32>())
        .map(|mass| mass / 3 - 2)
        .sum()
}

fn part_two(input: &str) -> i32
{
    input.lines()
        .flat_map(|s| s.parse::<i32>())
        .map(fuel_needed)
        .sum()
}

fn fuel_needed(mass: i32) -> i32
{
    let fuel = mass / 3 - 2;
    if fuel <= 0 { 0 } else { fuel + fuel_needed(fuel) }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 3317970);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 4974073);
    }

    #[test]
    fn example_part_one()
    {
        assert_eq!(fuel_needed(12), 2);
        assert_eq!(fuel_needed(14), 2);
        assert_eq!(fuel_needed(1969), 966);
        assert_eq!(fuel_needed(100756), 50346);
    }
}