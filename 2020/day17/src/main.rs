use std::collections::HashSet;

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

fn load(input: &str) -> HashSet<Point> {
    input.lines()
        .enumerate()
        .flat_map(|(y, l)| l.as_bytes().iter()
            .enumerate()
            .filter_map(move |(x, &b)|
                (b == b'#').then_some(Point(x as i32, y as i32, 0, 0))
            )
        ).collect()
}

fn part_one(input: &str) -> usize {
    use itertools::Itertools;

    let cubes = load(input);
    let mut deltas = (-1..=1)
        .map(|_| -1..=1)
        .multi_cartesian_product()
        .map(|mut v| { v.push(0); v })
        .collect::<Vec<_>>();
    let home = deltas.iter().find_position(|v| is_home(v)).unwrap().0;
    deltas.remove(home);

    (0..6).fold(cubes.clone(), |acc, _| cycle(&acc, &deltas)).len()
}

fn part_two(input: &str) -> usize {
    use itertools::Itertools;

    let cubes = load(input);
    let mut deltas = (-1..=1)
        .flat_map(|w| (-1..=1)
            .map(|_| -1..=1)
            .multi_cartesian_product()
            .map(|mut v| { v.push(w); v })
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    let home = deltas.iter().find_position(|v| is_home(v)).unwrap().0;
    deltas.remove(home);

    (0..6).fold(cubes.clone(), |acc, _| cycle(&acc, &deltas)).len()
}

fn is_home(v: &[i32]) -> bool {
    v.iter().all(|&n| n == 0)
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point(i32, i32, i32, i32);

fn cycle(cubes: &HashSet<Point>, deltas: &[Vec<i32>]) -> HashSet<Point> {
    use std::collections::VecDeque;

    let mut active = HashSet::new();
    let mut queue = cubes.iter().cloned().collect::<VecDeque<_>>();
    while let Some(pt) = queue.pop_back() {
        let nearby = deltas.iter().map(|v|
            Point(pt.0 + v[0], pt.1 + v[1], pt.2 + v[2], pt.3 + v[3])
        ).collect::<Vec<_>>();

        let count = nearby.iter().filter(|&p| cubes.contains(p)).count();
        if cubes.contains(&pt) {
            nearby.iter()
                .filter(|p| !cubes.contains(p))
                .for_each(|p| queue.push_back(*p));
 
            if count == 2 || count == 3 {
                active.insert(pt);
            }
        } else if count == 3 {
            active.insert(pt);
        }
    }
    
    active
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 319);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 2324);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 112);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 848);
    }
}