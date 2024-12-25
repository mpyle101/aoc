use std::collections::{HashMap, HashSet};

type TrailMap = HashMap<i32, (char, Vec<i32>)>;
type TrailGraph = HashMap<i32, HashSet<(i32, i32)>>;

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

fn part_one(input: &str) -> i32
{
    let (start, goal, ncols, trail) = load_trail_map(input);

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
//    use rayon::prelude::*;

    // A key insight is: because the paths are one step
    // wide and you can't backtrack, what you essentially
    // have is a graph with the start, end and intersection
    // positions as vertices and the paths between them as
    // edges. The weight of the edges is the number of steps
    // it takes to get there from position to the next. This
    // gives us a much, much smaller weighted graph to walk
    // when trying to find the longest simple path.
    let (start, goal, ncols, trail) = load_trails(input);
    let graph = build_graph(start, goal, ncols, &trail);
    longest_path(&graph, start, goal)
}

fn longest_path(graph: &TrailGraph, start: i32, goal: i32) -> i32
{
    let mut seen = HashSet::new();
    let mut count = 0;

    // Exhaustive depth first search.
    edfs(graph, goal, start, 0, &mut seen, &mut count)
}

fn edfs(
    graph: &TrailGraph,
    goal: i32,
    node: i32,
    steps: i32,
    seen: &mut HashSet<i32>,
    count: &mut i32) -> i32
{
    if node == goal {
        *count = (*count).max(steps)
    } else if !seen.contains(&node) {
        seen.insert(node);
        for (n, c) in graph.get(&node).unwrap() {
            edfs(graph, goal, *n, steps + c, seen, count);
        }
        seen.remove(&node);     // exhaustive
    }

    *count
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

fn load_trail_map(input: &str) -> (i32, i32, i32, TrailMap)
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

fn load_trails(input: &str) -> (i32, i32, i32, HashSet<i32>)
{
    let mut start = i32::MAX;
    let mut goal = 0;

    let mut ncols = 0;
    let mut trail = HashSet::new();

    input.lines()
        .zip(0..)
        .for_each(|(line, row)| {
            ncols = line.len() as i32;
            line.chars()
                .zip(0..)
                .filter(|(c, _)| *c != '#')
                .for_each(|(_, col)| {
                    let pos = row * ncols + col;
                    goal = pos;
                    if start == i32::MAX { start = pos }
                    trail.insert(pos);
                });
            });

    (start, goal, ncols, trail)
}

#[derive(Debug)]
struct Walk {
    pos: i32,
    last: i32,
    node: i32,
    steps: i32,
    visited: HashSet<i32>,
}

fn build_graph(
    start: i32,
    goal: i32,
    ncols: i32,
    trail: &HashSet<i32>
) -> TrailGraph
{
    use std::collections::VecDeque;

    let mut graph = TrailGraph::from([(start, HashSet::new())]);

    let mut stack = VecDeque::from([
        Walk { 
            pos: start,
            last: start,
            node: start,
            steps: 0,
            visited: HashSet::from([start])
        }
    ]);
    while let Some(mut w) = stack.pop_back() {
        if w.pos == goal {
            graph.entry(w.node).or_default().insert((goal, w.steps));
        } else {
            let steps = [
                w.pos - 1,      // left
                w.pos + 1,      // right
                w.pos - ncols,  // up
                w.pos + ncols   // down
            ].into_iter()
                .filter(|p| *p != w.last && trail.contains(p))
                .collect::<Vec<_>>();

            if steps.len() == 1 {
                let pos = steps[0];
                if w.visited.insert(pos) {
                    w.last = w.pos;
                    w.pos = pos;
                    w.steps += 1;
                    stack.push_back(w)
                }
            } else if graph.entry(w.node).or_default().insert((w.pos, w.steps)) {
                stack.extend(steps.iter()
                    .filter(|pos| !w.visited.contains(pos))
                    .map(|&pos| {
                        let mut visited = w.visited.clone();
                        visited.insert(pos);
                        Walk { pos, last: w.pos, node: w.pos, steps: 1, visited }
                    }))
            }
        }
    }

    graph
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
