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
    use std::collections::HashSet;

    let houses = input.chars()
        .scan((0, 0), |st, c| {
            *st = step(c, *st);
            Some(*st)
        })
        .collect::<HashSet<_>>();

    houses.len() + 1
}

fn part_two(input: &str) -> usize
{
    use std::collections::HashSet;

    let mut pos = [(0, 0), (0, 0)];
    let mut houses = input.chars()
        .enumerate()
        .map(|(i, c)| {
            let house = &mut pos[i % 2];
            *house = step(c, *house);
            *house
        })
        .collect::<HashSet<_>>();
    houses.insert((0, 0));

    houses.len()
}

fn step(c: char, (x, y): (i32, i32)) -> (i32, i32) {
    match c {
        '>' => (x + 1, y),
        '<' => (x - 1, y),
        '^' => (x, y - 1),
        'v' => (x, y + 1),
         _  => unreachable!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 2081);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 2341);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two("^v"), 3);
        assert_eq!(part_two("^>v<"), 3);
        assert_eq!(part_two("^v^v^v^v^v"), 11);
    }
}