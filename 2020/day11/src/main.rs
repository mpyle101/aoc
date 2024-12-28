use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

type NeighborsFn = fn(usize, &[char], i32, i32) -> usize;

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
    board(4, input, adjacent)
}

fn part_two(input: &str) -> usize
{
    board(5, input, visible)
}

fn board(n: usize, input: &str, neighbors: NeighborsFn) -> usize
{
    use std::collections::HashSet;

    let (nrows, ncols, mut seats) = load(input);

    let mut seen = HashSet::new();
    while seen.insert(hash(&seats)) {
        seats = update(n, &seats, ncols, nrows, neighbors);
    }

    seats.iter()
        .filter(|c| **c == '#')
        .count()
}

fn update(n: usize, seats: &[char], ncols: i32, nrows: i32, neighbors: NeighborsFn) -> Vec<char>
{
    seats.iter()
        .enumerate()
        .map(|(p, &c)| {
            let count = neighbors(p, seats, ncols, nrows);
            match c {
                'L' if count == 0 => '#',
                '#' if count >= n => 'L',
                 _ => c
            }
        })
        .collect()
}

static SEATS: [(i32, i32);8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

fn adjacent(p: usize, seats: &[char], ncols: i32, nrows: i32) -> usize
{
    let p = p as i32;
    let row = p / ncols;
    let col = p % ncols;
    let in_bounds = |(r, c): &(i32, i32)| (0..nrows).contains(r) && (0..ncols).contains(c);

    SEATS.iter()
        .map(|(dr, dc)| (row + dr, col + dc))
        .filter(in_bounds)
        .map(|(row, col)| row * ncols + col)
        .filter(|&p| seats[p as usize] == '#')
        .count()
}

fn visible(p: usize, seats: &[char], ncols: i32, nrows: i32) -> usize
{
    let p = p as i32;
    let row = p / ncols;
    let col = p % ncols;
    let in_bounds = |(r, c): &(i32, i32)| (0..nrows).contains(r) && (0..ncols).contains(c);

    SEATS.iter()
        .map(|(dr, dc)| ((dr, dc), row + dr, col + dc))
        .filter(|((&dr, &dc), mut row, mut col)| {
            let mut occupied = false;
            while in_bounds(&(row, col)) {
                let i = row * ncols + col;
                let c = seats[i as usize];
                if c != '.' { occupied = c == '#'; break }
                row += dr; col += dc;
            }
            occupied
        })
        .count()
}

fn hash(v: &[char]) -> u64
{
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}

fn load(input: &str) -> (i32, i32, Vec<char>)
{
    let mut nrows = 0;
    let mut ncols = 0;
    let seats = input.lines()
        .fold(vec![], |mut v, line| {
            nrows += 1;
            ncols = line.len() as i32;
            v.extend(line.chars());
            v
    });

    (nrows, ncols, seats)
}

#[allow(dead_code)]
fn print(seats: &[char], ncols: i32)
{
    seats.iter()
        .zip(0..)
        .for_each(|(c, i)| {
            if i % ncols == 0 { println!() }
            print!("{c}")
        });
    println!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 2344);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 2076);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 37);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 26);
    }

}