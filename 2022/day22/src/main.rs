use std::collections::HashMap;
use lazy_static::lazy_static;

const N: usize = 50;

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let password = part_one(input);
    println!("Part 1: {} ({:?})", password, t.elapsed());

    let t = Instant::now();
    let password = part_two(input);
    println!("Part 2: {} ({:?})", password, t.elapsed());
}

#[allow(dead_code)]
fn part_one(input: &str) -> usize {
    let (tiles, actions) = load(input);

    // right: 0, down: 1, left: 2, up: 3
    let mut facing = 0i32;
    let mut pos = (0, tiles[0].iter().take_while(|c| **c == ' ').count());
    actions.iter()
        .for_each(|a| {
            match a {
                Action::Move(n)   => pos = move_2d(facing, pos, *n, &tiles),
                Action::TurnLeft  => facing = (facing - 1).rem_euclid(4),
                Action::TurnRight => facing = (facing + 1).rem_euclid(4),
            }
        });

    1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + facing as usize
}

fn part_two(input: &str) -> usize {
    let (tiles, actions) = load(input);
    let faces = vec![(0, N), (0, 2*N), (N, N), (2*N, 0), (2*N, N), (3*N, 0)];

    // right: 0, down: 1, left: 2, up: 3
    let mut pos = (0, 0i32, 0, 0);
    actions.iter()
        .for_each(|a| {
            pos = match a {
                Action::Move(n)   => move_3d(pos, *n, &tiles, &faces),
                Action::TurnLeft  => (pos.0, (pos.1 - 1).rem_euclid(4), pos.2, pos.3),
                Action::TurnRight => (pos.0, (pos.1 + 1).rem_euclid(4), pos.2, pos.3),
            };
        });

    let row = pos.2 + 1 + faces[pos.0].0;
    let col = pos.3 + 1 + faces[pos.0].1;

    1000 * row + 4 * col + pos.1 as usize
}

fn load(input: &str) -> (Vec<Vec<char>>, Vec<Action>) {
    use regex::Regex;

    let (tiles, dirs) = input.split_once("\n\n").unwrap();
    let mut tiles = tiles.lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let cols = tiles.iter().map(|r| r.len()).max().unwrap();
    tiles.iter_mut()
        .for_each(|v| v.resize_with(cols, || ' '));

    let re = Regex::new(r"\d+|[RL]").unwrap();
    let actions = re.find_iter(dirs)
        .map(|m| Action::new(m.as_str()))
        .collect::<Vec<_>>();

    (tiles, actions)
}

fn move_2d(
    dir: i32,
    mut pos: (usize, usize),
    mut steps: i32,
    map: &[Vec<char>],
) -> (usize, usize) {  
    let (mut row, mut col) = next_tile_2d(dir, pos, map);
    while steps > 0 {
        let c = map[row][col];
        if c == '.' {
            steps -= 1;
            pos = (row, col);
            (row, col) = next_tile_2d(dir, pos, map);
        } else if c == '#' {
            break
        } else {
            (row, col) = next_tile_2d(dir, (row, col), map);
        }
    }

    pos
}

const OFFSETS: [(i32, i32);4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn next_tile_2d(dir: i32, (row, col): (usize, usize), map: &[Vec<char>]) -> (usize, usize) {
    let (delta_r, delta_c) = OFFSETS[dir as usize];
    (
        (row as i32 + delta_r).rem_euclid(map.len() as i32) as usize,
        (col as i32 + delta_c).rem_euclid(map[0].len() as i32) as usize,
    )
}

fn move_3d(
    mut pos: (usize, i32, usize, usize),
    mut steps: i32,
    map: &[Vec<char>],
    faces: &[(usize, usize)]
) -> (usize, i32, usize, usize) {
    let (mut face, mut dir, mut row, mut col) = next_tile_3d(pos);
    let mut c = map[row + faces[face].0][col + faces[face].1];

    while steps > 0 && c != '#' {
        steps -= 1;
        pos = (face, dir, row, col);

        (face, dir, row, col) = next_tile_3d(pos);
        c = map[row + faces[face].0][col + faces[face].1];
    }

    pos
}

fn next_tile_3d((face, dir, row, col): (usize, i32, usize, usize)) -> (usize, i32, usize, usize)
{
    let (delta_r, delta_c) = OFFSETS[dir as usize];
    let row_next = row as i32 + delta_r;
    let col_next = col as i32 + delta_c;

    if !(0..N as i32).contains(&row_next) {
        let (f, d, r, c) = *FACES.get(&(face, dir)).unwrap();
        (f, d, r(col), c(col))
    } else if !(0..N as i32).contains(&col_next) {
        let (f, d, r, c) = *FACES.get(&(face, dir)).unwrap();
        (f, d, r(row), c(row))
    } else {
        (face, dir, row_next as usize, col_next as usize)
    }
}

const X: fn(usize) -> usize = |x| x;
const Z: fn(usize) -> usize = |_| 0;
const W: fn(usize) -> usize = |_| N - 1;
const M: fn(usize) -> usize = |x| N - 1 - x;

type Transition = (usize, i32, fn(usize) -> usize, fn(usize) -> usize);
lazy_static! {
    static ref FACES: HashMap<(usize, i32), Transition> = 
        HashMap::from([
            ((0, 0), (1, 0, X, Z)), ((0, 1), (2, 1, Z, X)), ((0, 2), (3, 0, M, Z)), ((0, 3), (5, 0, X, Z)),
            ((1, 0), (4, 2, M, W)), ((1, 1), (2, 2, X, W)), ((1, 2), (0, 2, X, W)), ((1, 3), (5, 3, W, X)),
            ((2, 0), (1, 3, W, X)), ((2, 1), (4, 1, Z, X)), ((2, 2), (3, 1, Z, X)), ((2, 3), (0, 3, W, X)),
            ((3, 0), (4, 0, X, Z)), ((3, 1), (5, 1, Z, X)), ((3, 2), (0, 0, M, Z)), ((3, 3), (2, 0, X, Z)),
            ((4, 0), (1, 2, M, W)), ((4, 1), (5, 2, X, W)), ((4, 2), (3, 2, X, W)), ((4, 3), (2, 3, W, X)),
            ((5, 0), (4, 3, W, X)), ((5, 1), (1, 1, Z, X)), ((5, 2), (0, 1, Z, X)), ((5, 3), (3, 3, W, X)),
        ]);
}


#[derive(Debug)]
enum Action {
    Move(i32),
    TurnLeft,
    TurnRight,
}

impl Action {
    fn new(s: &str) -> Action {
        use Action::*;

        match s {
            "L" => TurnLeft,
            "R" => TurnRight,
            _ => Move(s.parse::<i32>().unwrap())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");

        let password = part_one(input);
        assert_eq!(password, 191010);

        let password = part_two(input);
        assert_eq!(password, 55364);
    }

    #[test]
    fn example() {
        let input = include_str!("../example.txt");

        let password = part_one(input);
        assert_eq!(password, 6032);

        // let password = part_one(input);
        // assert_eq!(password, 5031);
    }
}
