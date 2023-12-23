use std::collections::HashMap;

type TrailMap = HashMap<i32, (char, Vec<i32>)>;

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

fn part_one(input: &str) -> i32
{
    let (start, goal, ncols, trail) = load(input, true);

    let mut steps = 0;
    let mut q = vec![State::new(start)];
    while !q.is_empty() {
        if let Some(s) = q.iter()
            .filter(|st| st.pos == goal)
            .map(|st| st.steps)
            .max() {
                steps = steps.max(s)
            }

        q = q.iter()
            .filter(|st| st.pos != goal)
            .flat_map(|st| step(st, ncols, &trail))
            .collect();
    }

    steps - 1
}

fn part_two(input: &str) -> i32
{
    use rayon::prelude::*;

    let (start, goal, ncols, trail) = load(input, false);

    let mut steps = 0;
    let mut q = vec![State::new(start)];
    while !q.is_empty() {
        if let Some(s) = q.par_iter()
            .filter(|st| st.pos == goal)
            .map(|st| st.steps)
            .max() {
                steps = steps.max(s)
            }

        q = q.par_iter()
            .filter(|st| st.pos != goal)
            .flat_map(|st| step(st, ncols, &trail))
            .collect();
    }

    steps - 1
}

fn step(state: &State, ncols: i32, trail: &TrailMap) -> Vec<State>
{
    let (c, tiles) = trail.get(&state.pos).unwrap();
    
    if *c == '.' {
        let intersection = tiles.len() > 2;
        tiles.iter()
            .filter(|&p| *p != state.last && !state.tiles.contains(p))
            .map(|&pos| {
                let mut path = state.tiles.clone();
                if intersection { path.push(state.pos) };
                State { pos, last: state.pos, steps: state.steps + 1, tiles: path }
            })
            .collect()
    } else {
        let pos = match c {
            '>' => state.pos + 1,
            '<' => state.pos - 1,
            '^' => state.pos - ncols,
            'v' => state.pos + ncols,
            _ => panic!("Unknown tile type: {c}")
        };

        if pos == state.last || state.tiles.contains(&pos) {
            vec![]
        } else {
            let tiles = state.tiles.clone();
            vec![State { pos, last: state.pos, steps: state.steps + 1, tiles }]
        }
    }
}

fn load(input: &str, slippery: bool) -> (i32, i32, i32, TrailMap)
{
    let mut start = i32::MAX;
    let mut goal = 0;

    let mut ncols = 0;
    let mut trail = TrailMap::new();

    input.lines()
        .zip(0..)
        .for_each(|(line, row)| {
            ncols = line.len() as i32;
            line.chars()
                .zip(0..)
                .filter(|(c, _)| *c != '#')
                .for_each(|(ch, col)| {
                    let pos = row * ncols + col;
                    goal = pos;
                    if start == i32::MAX { start = pos }
                    let c = if !slippery { '.' } else { ch };

                    let mut v = vec![];
                    if let Some(p) = trail.get_mut(&(pos - 1)) {
                        if c == '.' || c == '<' {
                            v.push(pos - 1);
                        }
                        if p.0 == '.' || p.0 == '>' {
                            p.1.push(pos);
                        }
                    }
                    if let Some(p) = trail.get_mut(&(pos - ncols)) {
                        if c == '.' || c == '^' {
                            v.push(pos - ncols);
                        }
                        if p.0 == '.' || p.0 == 'v' {
                            p.1.push(pos);
                        }
                    }
                    trail.insert(pos, (c, v));
                })
        });

    (start, goal, ncols, trail)
}


#[derive(Clone, Eq, PartialEq)]
struct State {
    pos: i32,
    last: i32,
    steps: i32,
    tiles: Vec<i32>,
}
impl State {
    fn new(pos: i32) -> Self
    {
        State { pos, last: -1, steps: 1, tiles: vec![] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 2334);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 6422);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 94);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 154);
    }
}
