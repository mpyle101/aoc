use std::collections::{HashMap, HashSet};

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let empty = part_one(input);
    println!("Part 1: {} ({:?})", empty, t.elapsed());

    let t = Instant::now();
    let round = part_two(input);
    println!("Part 2: {} ({:?})", round, t.elapsed());
}

fn part_one(input: &str) -> u32 {
    let mut dir = 0;
    let mut elves = load(input);

    for _ in 1..=10 {
        let mut moved = HashSet::new();
        proposed_moves(dir, &elves).iter()
            .filter(|(_, v)| v.len() == 1 && moved.insert(v[0]))
            .map(|(p, v)| (v[0], p))
            .for_each(|(src, dst)| {
                elves.remove(&src);
                elves.insert(*dst);
            });

        dir = (dir + 1) % 4;
    }

    let (min_r, min_c, max_r, max_c) = elves.iter()
        .fold((i32::MAX, i32::MAX, i32::MIN, i32::MIN), |acc, pt| {
            (pt.0.min(acc.0), pt.1.min(acc.1), pt.0.max(acc.2), pt.1.max(acc.3))
        });
    let rows = min_r.abs_diff(max_r) + 1;
    let cols = min_c.abs_diff(max_c) + 1;

    rows * cols - elves.len() as u32
}

fn part_two(input: &str) -> u32 {
    let mut dir = 0;
    let mut elves = load(input);

    let mut round = 1;
    loop {
        let mut moved = HashSet::new();
        proposed_moves(dir, &elves).iter()
            .filter(|(_, v)| v.len() == 1 && moved.insert(v[0]))
            .map(|(p, v)| (v[0], p))
            .for_each(|(src, dst)| {
                elves.remove(&src);
                elves.insert(*dst);
            });

        if moved.is_empty() {
            return round
        }

        round += 1;
        dir = (dir + 1) % 4;
    }
}

fn load(input: &str) -> HashSet<(i32, i32)> {
    input.lines()
        .enumerate()
        .flat_map(|(row, s)| s.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(move |(col, _)| (row as i32, col as i32)))
        .collect::<HashSet<_>>()
}

fn proposed_moves(dir: usize, elves: &HashSet<(i32, i32)>) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let mut proposed: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    elves.iter()
        .filter_map(|e| can_move(e, elves, dir).map(|p| (e, p)))
        .for_each(|(e, p)| proposed.entry(p).or_default().push(*e));

    proposed
}

fn can_move(elf: &(i32, i32), elves: &HashSet<(i32, i32)>, dir: usize) -> Option<(i32, i32)> {
    // If there are no adjacent elves, stay put.
    if DIRS.iter()
        .map(|(dr, dc)| (elf.0 + dr, elf.1 + dc))
        .all(|p| !elves.contains(&p))
    {
        return None
    }

    // Check each direction.
    for i in 0..4 {
        let ix = (dir + i) % 4;
        if LOOK[ix].iter()
            .map(|i| DIRS[*i])
            .map(|(dr, dc)| (elf.0 + dr, elf.1 + dc))
            .all(|p| !elves.contains(&p))
        {
            // Return the move in first valid direction.
            let (dr, dc) = DIRS[LOOK[ix][1]];
            return Some((elf.0 + dr, elf.1 + dc))
        }
    }

    None
}

const DIRS: [(i32, i32);8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

// N, S, W, E
const LOOK: [[usize;3];4] = [
    [0, 1, 2], [5, 6, 7], [0, 3, 5], [2, 4, 7]
];


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let empty = part_one(input);
        assert_eq!(empty, 4082);

        let round = part_two(input);
        assert_eq!(round, 1065);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let empty = part_one(input);
        assert_eq!(empty, 100);

        let round = part_two(input);
        assert_eq!(round, 20);
    }
}
