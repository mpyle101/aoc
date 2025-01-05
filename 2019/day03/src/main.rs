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
    use std::iter::FromIterator;
    use std::collections::HashSet;
    type Path = HashSet<(i32, i32)>;

    let (s1, s2) = input.split_once('\n').unwrap();
    let a = Path::from_iter(generate_path(s1));
    let b = Path::from_iter(generate_path(s2));

    (&a & &b).iter()
        .map(|p| md(*p, (0, 0)))
        .min()
        .unwrap()
}

fn part_two(input: &str) -> usize
{
    use std::iter::FromIterator;
    use std::collections::HashSet;
    type Path = HashSet<(i32, i32)>;

    let (s1, s2) = input.split_once('\n').unwrap();
    let w1 = generate_path(s1);
    let w2 = generate_path(s2);
    let a = Path::from_iter(w1.clone());
    let b = Path::from_iter(w2.clone());

    (&a & &b).iter()
        .flat_map(|p| w1.iter().position(|p1| p1 == p).map(|i| (p, i + 1)))
        .flat_map(|(p, i)| w2.iter().position(|p1| p1 == p).map(|j| i + j + 1))
        .min()
        .unwrap()
}

fn md((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u32
{
    x2.abs_diff(x1) + y2.abs_diff(y1)
}

fn generate_path(wire: &str) -> Vec<(i32, i32)>
{
    let mut path = vec![];
    wire.split(',')
        .map(|s| {
            let (dx, dy) = match s.chars().next() {
                Some('R') => ( 1,  0),
                Some('L') => (-1,  0),
                Some('U') => ( 0, -1),
                Some('D') => ( 0,  1),
                _ => unreachable!()
            };
            let n = s[1..].parse::<i32>().unwrap();
            (dx, dy, n)
        })
        .fold((0, 0), |(mut x, mut y), (dx, dy, n)| {
            (0..n).for_each(|_| {
                x += dx; y += dy;
                path.push((x, y));
            });
            (x, y)
        });

    path
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 386);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 6484);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 159);

        let input = include_str!("../example2.txt");
        assert_eq!(part_one(input), 135);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_two(input), 610);

        let input = include_str!("../example2.txt");
        assert_eq!(part_two(input), 410);
    }

}