use std::collections::{HashMap, HashSet};

type Route<'a>  = (&'a str, &'a str);
type Routes<'a> = HashMap<Route<'a>, u32>;

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
    let routes = load(input);
    let cities = routes.keys()
        .map(|(a, _)| *a)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    cities.iter()
        .enumerate()
        .map(|(i, city)| {
            let visited = 1 << i;
            shortest_path(city, visited, &cities, &routes)
        })
        .min()
        .unwrap()
}

fn part_two(input: &str) -> u32
{
    let routes = load(input);
    let cities = routes.keys()
        .map(|(a, _)| *a)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    cities.iter()
        .enumerate()
        .map(|(i, city)| {
            let visited = 1 << i;
            longest_path(city, visited, &cities, &routes)
        })
        .max()
        .unwrap()
}

fn shortest_path(city: &str, visited: u32, cities: &[&str], routes: &Routes) -> u32
{
    if visited.count_ones() == cities.len() as u32 {
        0
    } else {
        cities.iter()
            .enumerate()
            .filter(|(i, _)| (visited & 1 << i) == 0)
            .flat_map(|(i, c)| routes.get(&(city, c)).map(|n| (i, c, *n)))
            .map(|(i, c, n)| {
                let visited = visited | (1 << i);
                n + shortest_path(c, visited, cities, routes)
            })
            .min()
            .unwrap_or(u32::MAX)
    }
}

fn longest_path(city: &str, visited: u32, cities: &[&str], routes: &Routes) -> u32
{
    if visited.count_ones() == cities.len() as u32 {
        0
    } else {
        cities.iter()
            .enumerate()
            .filter(|(i, _)| (visited & 1 << i) == 0)
            .flat_map(|(i, c)| routes.get(&(city, c)).map(|n| (i, c, *n)))
            .map(|(i, c, n)| {
                let visited = visited | (1 << i);
                n + longest_path(c, visited, cities, routes)
            })
            .max()
            .unwrap_or(0)
    }
}

fn load(input: &str) -> Routes
{
    input.lines()
        .map(|line| line.split(' '))
        .flat_map(|mut iter| {
            let c1 = iter.next().unwrap();
            iter.next();    // to
            let c2 = iter.next().unwrap();
            iter.next();    // =
            let n = iter.next().unwrap().parse::<u32>().unwrap();
            [((c1, c2), n), ((c2, c1), n)]
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
        assert_eq!(part_one(input), 141);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 736);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 605);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 982);
    }
}