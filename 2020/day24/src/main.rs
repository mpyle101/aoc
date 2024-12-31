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

fn part_one(input: &str) -> usize
{
    let tiles = load(input);

    tiles.len()
}

fn part_two(input: &str) -> usize
{
    let tiles = load(input);
    (0..100).fold(tiles, |tiles, _| {
        let mut white = HashSet::new();
        let mut black = HashSet::new();

        tiles.iter().for_each(|tile| {
            let adjacent = NEIGHBORS.iter()
                .map(|(dx, dy)| (tile.0 + dx, tile.1 + dy))
                .fold(0, |acc, p|
                    if tiles.contains(&p) { 
                        acc + 1
                    } else {
                        white.insert(p);
                        acc
                    }
                );
            if adjacent == 1 || adjacent == 2 {
                black.insert(*tile);
            }
        });
        white.iter()
            .filter(|tile| adjacent(&tiles, tile) == 2)
            .for_each(|tile| { black.insert(*tile); });

        black
    })
    .len()
}

fn load(input: &str) -> HashSet<(i32, i32)>
{
    use std::collections::HashMap;

    input.lines()
        .fold(HashMap::new(), |mut acc, s| {
            let (tile, _) = s.chars().fold(((0, 0), '_'), |((x, y), p), c|
                (match (p, c) {
                    // Doubled coordinates
                    ('n', 'e') => (x + 1, y - 1),
                    ('s', 'e') => (x + 1, y + 1),
                    ('n', 'w') => (x - 1, y - 1),
                    ('s', 'w') => (x - 1, y + 1),
                    ( _ , 'e') => (x + 2, y),
                    ( _ , 'w') => (x - 2, y),
                            _ => (x, y)
                }, c)
            );
            *acc.entry(tile).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter(|(_, &v)| v % 2 != 0)
        .map(|(p, _)| *p)
        .collect()
}

const NEIGHBORS: [(i32, i32); 6] = [
    (2, 0), (-2, 0), (1, -1), (-1, -1), (1, 1), (-1, 1)
];

fn adjacent(black: &HashSet<(i32, i32)>, (x, y): &(i32, i32)) -> usize {
    NEIGHBORS.iter().filter(|(dx, dy)| black.contains(&(x + dx, y + dy))).count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 254);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 3697);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 10);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 2208);
    }
}