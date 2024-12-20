fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    use pathfinding::prelude::dijkstra;

    let (start, goal, ncols, mut maze) = load(input);
    let (_, cost) = dijkstra(
        &start,
        |&p| do_moves(p, ncols, &maze).into_iter().map(|p| (p, 1)),
        |&p| p == goal
    ).unwrap();

    let walls = (ncols + 1..maze.len() - ncols)
        .filter(|&p| {
            let col = p % ncols;
            col > 0 && col < ncols - 1  && maze[p] == '#'
        })
        .collect::<Vec<_>>();

    let mut v = vec![];
    for p in walls {
        maze[p] = '.';
        if let Some(n) = find_path(cost - 100, start, goal, ncols, &maze) {
            v.push(n)
        }
        maze[p] = '#';
    }

    v.len()
}

fn find_path(limit: usize, start: usize, goal: usize, ncols: usize, maze: &[char]) -> Option<usize>
{
    use utils::dijkstra::dijkstra_limited;

    dijkstra_limited(
        &start,
        limit,
        |&p| do_moves(p, ncols, maze).into_iter().map(|p| (p, 1)),
        |&p| p == goal
    )
    .map(|r| r.1)
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
        assert_eq!(part_one(input), 1372);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 2);
    }
}
