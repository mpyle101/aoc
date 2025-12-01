use std::collections::HashMap;

type Memo = HashMap<(i32, i32), i32>;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one((31, 39), input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(goal: (i32, i32), input: &str) -> usize
{
    use pathfinding::prelude::bfs;

    let num = input.parse::<i32>().unwrap();
    bfs(&(1, 1), |p| neighbors(p, num), |&p| p == goal)
        .unwrap()
        .len() - 1  // the vector contains the initial state.
}

fn part_two(input: &str) -> usize
{
    use pathfinding::prelude::bfs_reach;

    let num = input.parse::<i32>().unwrap();
    let mut memo = Memo::new();
    bfs_reach((1, 1), |p| reachable(p, num, &mut memo)).count()
}

fn neighbors((x, y): &(i32, i32), num: i32) -> Vec<(i32, i32)>
{
    [(0, -1), (0, 1), (-1, 0), (1, 0)]
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .filter(|p| is_open(*p, num))
        .collect()
}

fn reachable((x, y): &(i32, i32), num: i32, memo: &mut Memo) -> Vec<(i32, i32)>
{
    let n = *memo.entry((*x, *y)).or_default();
    if n < 50 {
        [(0, -1), (0, 1), (-1, 0), (1, 0)]
            .iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .filter(|p| is_open(*p, num))
            .inspect(|p| { memo.insert(*p, n + 1); })
            .collect()
    } else {
        vec![]
    }
}

fn is_open((x, y): (i32, i32), num: i32) -> bool
{
    let n = x.pow(2) + (3 * x) + (2 * x * y) + y + y.pow(2) + num;
    n.count_ones().is_multiple_of(2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one((31, 39), input), 92);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 124);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one((7, 4), input), 11);
    }
}