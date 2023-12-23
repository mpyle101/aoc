use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

type TrailMap = HashMap<i32, (char, Vec<i32>)>;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../example.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    use pathfinding::prelude::yen;

    let (start, goal, ncols, trail) = load(input);
    let res = yen(
        &State::new(start),
        |st| step(st, ncols, &trail).into_iter().map(|st| (st, 1)),
        |st| st.pos == goal,
        10
    );

    println!("{}", res.len());
    
    res.last().unwrap().1
}

fn step(state: &State, ncols: i32, trail: &TrailMap) -> Vec<State>
{
    let (c, tiles) = trail.get(&state.pos).unwrap();
    
    if *c == '.' {
        tiles.iter()
            .filter(|p| !state.path.contains(p))
            .map(|&pos| {
                let mut path = state.path.clone();
                path.insert(pos);
                State { pos, path }
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

        if state.path.contains(&pos) {
            vec![]
        } else {
            let mut path = state.path.clone();
            path.insert(pos);
            [State { pos, path }].to_vec()
        }
    }
}

fn load(input: &str) -> (i32, i32, i32, TrailMap)
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
                .for_each(|(c, col)| {
                    let pos = row * ncols + col;
                    goal = pos;
                    if start == i32::MAX { start = pos }

                    let mut v = vec![];
                    if let Some(p) = trail.get_mut(&(pos - 1)) {
                        if c == '<' || c == '.' {
                            v.push(pos - 1);
                        }
                        if p.0 == '>' || p.0 == '.' {
                            p.1.push(pos);
                        }
                    }
                    if let Some(p) = trail.get_mut(&(pos - ncols)) {
                        if c == '^' || c == '.' {
                            v.push(pos - ncols);
                        }
                        if p.0 == 'v' || p.0 == '.' {
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
    path: HashSet<i32>,
}
impl State {
    fn new(pos: i32) -> Self
    {
        State { pos, path: HashSet::from([pos]) }
    }
}
impl Hash for State {
    fn hash<H: Hasher>(&self, hasher: &mut H)
    {
        self.pos.hash(hasher);
        self.path.iter().for_each(|p| p.hash(hasher));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 345015);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 94);
    }
}
