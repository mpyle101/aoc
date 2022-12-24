type Pos = (i32, i32);
type Wind = (char, Pos);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    pos: Pos,
    time: i32,
}

struct Map {
    rows: i32,
    cols: i32,
}

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
    let start = State { pos: (0, 1), time: 0 };
    let path  = astar(
        &start,
        |st| neighbors(st, &wind, &map).into_iter().map(|p| (p, 1)),
        |st: &State| st.pos.0.abs_diff(goal.0) + st.pos.1.abs_diff(goal.1),
        |st: &State| st.pos == goal
    ).unwrap();

    path.0.len() - 1
}

#[allow(dead_code)]
fn part_two(input: &str) -> usize
{
    use pathfinding::prelude::astar;

    let (map, wind) = load(input);
    let goals = [
        (map.rows - 1, map.cols - 2),   // There...
        (0, 1),                         // And back...
        (map.rows - 1, map.cols - 2)    // And back...again
    ];

    let mut start = State { pos: (0, 1), time: 0 };
    (0..3).fold(0, |steps, i| {
        let goal = goals[i];
        let path = astar(
            &start,
            |st| neighbors(st, &wind, &map).into_iter().map(|p| (p, 1)),
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

    let mat = Matrix::from_rows(input.lines().map(|s| s.chars())).unwrap();
    let wind = mat.keys()
        .filter_map(|p| mat.get(p).map(|c| (c, p)))
        .filter(|(&c, _)| c != '.' && c != '#')
        .map(|(c, p)| (*c, (p.0 as i32 - 1, p.1 as i32 - 1)))
        .collect::<Vec<_>>();

    (Map { rows: mat.rows as i32, cols: mat.columns as i32 }, wind)
}

// up, down, left, right or wait
const DIRS: [Pos;5] = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];

fn neighbors(st: &State, wind: &[Wind], map: &Map) -> Vec<State>
{
    let states = DIRS.iter()
        .map(|(dr, dc)| (st.pos.0 + dr, st.pos.1 + dc))
        .filter(|pos| is_open(pos, map))
        .filter(|pos| wind.iter()
            .all(|(c, p)| blizzard(c, st.time + 1, p, map) != *pos)
        )
        .map(|pos| State { pos, time: st.time + 1 })
        .collect::<Vec<_>>();

    states
}

fn is_open(p: &Pos, m: &Map) -> bool
{
    ((1..m.rows-1).contains(&p.0) && (1..m.cols-1).contains(&p.1)) ||
        *p == (0, 1) ||
        *p == (m.rows - 1, m.cols - 2)
}

fn blizzard(dir: &char, t: i32, p: &Pos, m: &Map) -> (i32, i32)
{
    // Subtract 2 for the walls.
    let pt = match dir {
        '^' => ((p.0 - t).rem_euclid(m.rows - 2), p.1),
        'v' => ((p.0 + t).rem_euclid(m.rows - 2), p.1),
        '<' => (p.0, (p.1 - t).rem_euclid(m.cols - 2)),
        '>' => (p.0, (p.1 + t).rem_euclid(m.cols - 2)),
        _ => unreachable!()
    };

    // Offset to account for walls.
    (pt.0 + 1, pt.1 + 1)
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
