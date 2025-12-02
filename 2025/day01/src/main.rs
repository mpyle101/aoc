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
    let mut dial = 50;
    input.lines()
        .fold(0, |zeros, line| {
            let dir = line.chars().next().unwrap();
            let ticks = line[1..].parse::<u32>().unwrap();

            dial = if dir == 'R' {
                (dial + ticks) % 100
            } else {
                let ticks = ticks % 100;
                if ticks <= dial {
                    dial - ticks
                } else {
                    100 - (ticks - dial)
                }
            };

            zeros + (dial == 0) as u32
        })
}

fn part_two(input: &str) -> u32
{
    let mut dial = 50;
    input.lines()
        .fold(0, |mut zeros, line| {
            let dir = line.chars().next().unwrap();
            let ticks = line[1..].parse::<u32>().unwrap();
            zeros += ticks / 100;

            dial = if dir == 'R' {
                if dial + (ticks % 100) > 100 {
                    zeros += 1
                }
                (dial + ticks) % 100
            } else {
                let ticks = ticks % 100;
                if ticks == dial {
                    0
                } else if ticks < dial {
                    dial - ticks
                } else {
                    if dial != 0 { zeros += 1 };
                    100 - (ticks - dial)
                }
            };

            zeros + (dial == 0) as u32
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1097);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 7101);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 3);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 6);
    }
}
