
fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let fuel = part_one(input);
    println!("Part 1: {} ({:?})", fuel, t.elapsed());

    let t = Instant::now();
    let fuel = part_two(input);
    println!("Part 2: {} ({:?})", fuel, t.elapsed());
}

fn part_one(input: &str) -> i32 {
    input.lines()
        .flat_map(|s| s.parse::<i32>())
        .map(|mass| mass / 3 - 2)
        .sum()
}

fn part_two(input: &str) -> i32 {
    input.lines()
        .flat_map(|s| s.parse::<i32>())
        .map(fuel_needed)
        .sum()
}

fn fuel_needed(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 { 0 } else { fuel + fuel_needed(fuel) }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let fuel = part_one(input);
        assert_eq!(fuel, 3317970);

        let fuel = part_two(input);
        assert_eq!(fuel, 4974073);
    }

    #[test]
    fn examples() {
        assert_eq!(fuel_needed(12), 2);
        assert_eq!(fuel_needed(14), 2);
        assert_eq!(fuel_needed(1969), 966);
        assert_eq!(fuel_needed(100756), 50346);
    }
}
