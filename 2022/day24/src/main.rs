use std::collections::HashSet;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let steps = part_one(input);
    println!("Part 1: {} ({:?})", steps, t.elapsed());

    let t = Instant::now();
    let steps = part_two(input);
    println!("Part 2: {} ({:?})", steps, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    use pathfinding::prelude::astar;

    let (map, wind) = load(input);
    let goal  = ((map.rows - 1), (map.cols - 2));
    let start = State { pos: (0, 1), wind };
    let path = astar(
        &start,
        |st| neighbors(st, &map).into_iter().map(|p| (p, 1)).collect::<Vec<_>>(),
        |st: &State| st.pos.0.abs_diff(goal.0) + st.pos.1.abs_diff(goal.1),
        |st: &State| st.pos == goal
    ).unwrap();

    path.0.len() - 1
}

fn part_two(input: &str) -> usize
{
    use pathfinding::prelude::astar;

    let (map, wind) = load(input);
    let goals = [
        (map.rows - 1, map.cols - 2),   // There...
        (0, 1),                         // And back...
        (map.rows - 1, map.cols - 2)    // And back...again
    ];

    let mut start = State { pos: (0, 1), wind };
    (0..3).fold(0, |steps, i| {
        let goal = goals[i];
        let path = astar(
            &start,
            |st| neighbors(st, &map).into_iter().map(|p| (p, 1)).collect::<Vec<_>>(),
            |st: &State| st.pos.0.abs_diff(goal.0) + st.pos.1.abs_diff(goal.1),
            |st: &State| st.pos == goal
        ).unwrap();
        start = path.0.last().unwrap().clone();
        
        steps + path.0.len() - 1
    })
}

fn load(input: &str) -> (Map, Vec<Wind>)
{
    use pathfinding::matrix::Matrix;

    let m = Matrix::from_rows(input.lines().map(|s| s.chars())).unwrap();
    let mut wind = vec![];
    let mut walls = HashSet::new();
    m.keys()
        .filter_map(|p| m.get(p).map(|c| (c, p)))
        .filter(|(&c, _)| c != '.')
        .map(|(c, p)| (*c, (p.0 as i32, p.1 as i32)))
        .for_each(|(c, p)| if c == '#' { walls.insert(p); } else { wind.push((c, p)) });

    // So you can't move up from the start position.
    walls.insert((-1, 1));

    (Map { walls, rows: m.rows as i32, cols: m.columns as i32 }, wind)
}

// up, down, left, right or wait
const DIRS: [Pos;5] = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];

fn neighbors(st: &State, map: &Map) -> Vec<State>
{
    let wind = move_blizzards(&st.wind, map.rows, map.cols);
    let states = DIRS.iter()
        .map(|(dr, dc)| (st.pos.0 + dr, st.pos.1 + dc))
        .filter(|p| wind.iter().all(|(_, pw)| p != pw))
        .filter(|p| !map.walls.contains(p))
        .map(|pos| State { pos, wind: wind.clone() })
        .collect::<Vec<_>>();

    states
}

fn move_blizzards(wind: &[Wind], rows: i32, cols: i32) -> Vec<Wind>
{
    wind.iter()
        .map(|(w, (r, c))| (*w, match w {
            '^' => if *r == 1 { (rows - 2, *c) } else { (*r - 1, *c) },
            'v' => if *r == rows - 2 { (1, *c) } else { (*r + 1, *c) },
            '<' => if *c == 1 { (*r, cols - 2) } else { (*r, *c - 1) },
            '>' => if *c == cols - 2 { (*r, 1) } else { (*r, *c + 1) },
            _ => unreachable!()
        }))
        .collect()
}

type Pos = (i32, i32);
type Wind = (char, Pos);

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    pos: Pos,
    wind: Vec<Wind>,
}

struct Map {
    rows: i32,
    cols: i32,
    walls: HashSet<Pos>,
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn it_works()
    {
        let input = include_str!("../input.txt");

        let steps = part_one(input);
        assert_eq!(steps, 322);

        let steps = part_two(input);
        assert_eq!(steps, 974);
    }

    #[test]
    fn example()
    {
        let input = include_str!("../example.txt");

        let steps = part_one(input);
        assert_eq!(steps, 18);

        let steps = part_two(input);
        assert_eq!(steps, 54);
    }
}
