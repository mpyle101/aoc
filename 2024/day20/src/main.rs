fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input, 100);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str, limit: usize) -> usize
{
    use std::collections::HashMap;
    use pathfinding::prelude::dijkstra;

    let (start, goal, ncols, maze) = load(input);
    let (path, _) = dijkstra(
        &start,
        |&p| do_moves(p, ncols, &maze).into_iter().map(|p| (p, 1)),
        |&p| p == goal
    ).unwrap();
    let tiles = path.iter()
        .enumerate()
        .map(|(i, p)| (*p, i))
        .collect::<HashMap<_,_>>();

    // We know there's only one path from the problem statement so all we
    // really need to do is for each step find steps through walls which
    // are farther along the path. The tiles map gives us maze position to
    // index into the path which tells us if a given tile is farther along
    // and, thus, saves steps. We put the results in a mapped keyed by steps
    // saved and keep count.
    let counts = path.iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (i, p)| {
            [p - 1, p + 1, p - ncols, p + ncols].iter()
                .filter(|q| maze[**q] == '#')
                .filter_map(|q| tiles.get(&((q + q).wrapping_sub(*p))))
                .filter(|j| **j > i)
                .for_each(|j| {
                    // time saved is the number of steps removed minus 2
                    // for the stepping through the wall
                    let saved = j - i - 2;
                    *m.entry(saved).or_default() += 1;
                });
            m
        });

    // Only count the ones saving at least the limit amount of steps.
    counts.iter()
        .filter_map(|(saved, n)| (*saved >= limit).then_some(n))
        .sum()
}

fn do_moves(p: usize, ncols: usize, maze: &[char]) -> Vec<usize>
{
    let mut moves = Vec::with_capacity(4);
    if maze[p - 1] == '.' { moves.push(p - 1) }
    if maze[p + 1] == '.' { moves.push(p + 1) }
    if maze[p - ncols] == '.' { moves.push(p - ncols)}
    if maze[p + ncols] == '.' { moves.push(p + ncols) }

    moves
}

fn load(input: &str) -> (usize, usize, usize, Vec<char>)
{
    let mut start = 0;
    let mut goal  = 0;
    let mut ncols = 0;

    let mut maze = input.lines()
        .enumerate()
        .fold(vec![], |mut v, (row, line)| {
            ncols = line.len();
            line.chars()
                .enumerate()
                .for_each(|(col, c)| {
                    if c == 'E' {
                        goal = row * ncols + col
                    } else if c == 'S' {
                        start = row * ncols + col
                    }
                });
            v.extend(line.chars());
            v
        });
    maze[goal]  = '.';
    maze[start] = '.';

    (start, goal, ncols, maze)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input, 100), 1372);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input, 2), 44);
    }
}
