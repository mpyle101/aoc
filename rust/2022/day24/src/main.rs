use std::collections::HashSet;

type Pos = (i32, i32);
type Wind = (char, Pos);

#[derive(Clone, Copy, Debug, Eq)]
struct State {
    pos: Pos,
    time: i32,
    cycle: i32,
}
impl core::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        (self.time % self.cycle).hash(state);
    }
}
impl std::cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && 
            (self.time % self.cycle) == (other.time % other.cycle)
    }
}

struct Map {
    rows: i32,
    cols: i32,
}

fn main() {
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part_one(input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part_two(input), t.elapsed());
}

fn part_one(input: &str) -> i32 {
    use num::integer::Integer;
    use pathfinding::prelude::astar;

    let (map, wind) = load(input);
    let cycle  = (map.rows - 2).lcm(&(map.cols - 2));
    let goal   = ((map.rows - 1), (map.cols - 2));
    let start  = State { pos: (0, 1), time: 0, cycle };
    let ground = open_ground(&wind, &map, cycle);

    let path = astar(
        &start,
        |st| neighbors(st, &ground).into_iter().map(|p| (p, 1)),
        |st| st.pos.0.abs_diff(goal.0) + st.pos.1.abs_diff(goal.1),
        |st| st.pos == goal
    ).unwrap();

    path.0.last().unwrap().time
}

fn part_two(input: &str) -> i32 {
    use num::integer::Integer;
    use pathfinding::prelude::astar;

    let (map, wind) = load(input);
    let cycle = (map.rows - 2).lcm(&(map.cols - 2));
    let goals = [
        (map.rows - 1, map.cols - 2),   // There...
        (0, 1),                         // And back...
        (map.rows - 1, map.cols - 2)    // And back...again
    ];
    let ground = open_ground(&wind, &map, cycle);

    let start = State { pos: (0, 1), time: 0, cycle };
    let state = goals.iter()
        .fold(start, |start, goal| {
            let path = astar(
                &start,
                |st| neighbors(st, &ground).into_iter().map(|p| (p, 1)),
                |st| st.pos.0.abs_diff(goal.0) + st.pos.1.abs_diff(goal.1),
                |st| st.pos == *goal
            ).unwrap();
            
            *path.0.last().unwrap()
        });

    state.time
}

fn load(input: &str) -> (Map, Vec<Wind>) {
    use pathfinding::matrix::Matrix;

    let m = Matrix::from_rows(input.lines().map(|s| s.chars())).unwrap();
    let wind = m.keys()
        .filter_map(|p| m.get(p).map(|c| (c, p)))
        .filter(|(&c, _)| c != '.' && c != '#')
        .map(|(c, p)| (*c, (p.0 as i32 - 1, p.1 as i32 - 1)))
        .collect::<Vec<_>>();

    (Map { rows: m.rows as i32, cols: m.columns as i32 }, wind)
}

// up, down, left, right or wait
const DIRS: [Pos;5] = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];

fn neighbors(st: &State, ground: &[HashSet<Pos>]) -> Vec<State> {
    let time = ((st.time + 1) % st.cycle) as usize;

    DIRS.iter()
        .map(|(dr, dc)| (st.pos.0 + dr, st.pos.1 + dc))
        .filter(|pos| ground[time].contains(pos))
        .map(|pos| State { pos, time: st.time + 1, cycle: st.cycle })
        .collect()
}

fn open_ground(wind: &[Wind], map: &Map, cycle: i32) -> Vec<HashSet<Pos>> {
    use itertools::Itertools;

    // Calculate the open locations for a time cycle.
    // (minus 2 for the walls).
    let rows = map.rows - 2;
    let cols = map.cols - 2;

    let mut valley = (1..=rows)
        .cartesian_product(1..=cols)
        .collect::<HashSet<_>>();
    valley.insert((0, 1));
    valley.insert((map.rows - 1, map.cols - 2));

    (0..cycle)
        .fold(vec![], |mut v, t| {
            let mut ground = valley.clone();
            wind.iter()
                .map(|(c, p)| blizzard(c, t, p, rows, cols))
                .for_each(|p| { ground.remove(&p); });
            
            v.push(ground);
            v
        })
}

fn blizzard(dir: &char, t: i32, p: &Pos, rows: i32, cols: i32) -> (i32, i32) {
    let (r, c) = match dir {
        '^' => ((p.0 - t).rem_euclid(rows), p.1),
        'v' => ((p.0 + t).rem_euclid(rows), p.1),
        '<' => (p.0, (p.1 - t).rem_euclid(cols)),
        '>' => (p.0, (p.1 + t).rem_euclid(cols)),
        _ => unreachable!()
    };

    // Adjust to be in the valley.
    (r + 1, c + 1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 322);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 974);
    }

    #[test]
    fn example_part_one() {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 18);
    }

    #[test]
    fn example_part_two() {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 54);
    }
}
