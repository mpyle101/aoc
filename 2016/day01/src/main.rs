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
    let steps = load(input);
    let (x, y) = steps.iter()
        .fold((0, 0), |(x, y), (c, n)| {
            match c {
                // rotate the world
                'R' => (-y, x + n),
                'L' => ( y, n - x),
                _  => unreachable!()
            }
        });

    x.abs() + y.abs()
}

fn part_two(input: &str) -> i32
{
    use std::collections::HashSet;

    let steps = load(input);

    let (mut x, mut y) = (0_i32, 0_i32);
    let mut visited = HashSet::from([(0, 0)]);

    let mut dir = '^';
    for (c, n) in steps {
        dir = match (dir, c) {
            ('^', 'R') => '>', ('^', 'L') => '<',
            ('v', 'R') => '<', ('v', 'L') => '>',
            ('<', 'R') => '^', ('<', 'L') => 'v',
            ('>', 'R') => 'v', ('>', 'L') => '^',
                     _ => unreachable!()
        };
        let (dx, dy) = match dir {
            '^' => ( 0, -1),
            'v' => ( 0,  1),
            '<' => (-1,  0),
            '>' => ( 1,  0),
             _  => unreachable!()
        };
        for _ in 0..n {
            (x, y) = (x + dx, y + dy);
            if !visited.insert((x, y)) {
                return x.abs() + y.abs()
            }
        }
    }

    0
}

fn load(input: &str) -> Vec<(char, i32)>
{
    input.lines()
        .flat_map(|s| s.split(", "))
        .map(|s| {
            let c = s.chars().next().unwrap();
            let n = s[1..].parse::<i32>().unwrap();
            (c, n)
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 231);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 147);
    }
}