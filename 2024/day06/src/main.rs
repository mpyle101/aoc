use std::collections::HashSet;

type Obstacles = HashSet<(i32, i32)>;

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

fn part_one(input: &str) -> usize
{
    let mut ncols = 0;
    let mut nrows = 0;
    let mut guard = (0, 0);
    let mut obstacles = Obstacles::new();

    input.lines()
        .enumerate()
        .for_each(|(row, line)| {
            ncols = line.len() as i32;
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .for_each(|(col, c)| {
                    if c == '#' {
                        obstacles.insert((row as i32, col as i32));
                    } else {
                        guard = (row as i32, col as i32);
                    }
                });
            nrows += 1;
        });

    let mut dir = '^';
    let mut steps = HashSet::new();
    while is_inbounds(guard, nrows, ncols) {
        steps.insert(guard);
        (guard, dir) = step(guard, dir, &obstacles);
    }

    steps.len()
}

fn part_two(input: &str) -> usize
{
    use rayon::prelude::*;

    let mut ncols = 0;
    let mut nrows = 0;
    let mut guard = (0, 0);
    let mut obstacles = Obstacles::new();

    input.lines()
        .enumerate()
        .for_each(|(row, line)| {
            ncols = line.len() as i32;
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .for_each(|(col, c)| {
                    if c == '#' {
                        obstacles.insert((row as i32, col as i32));
                    } else {
                        guard = (row as i32, col as i32);
                    }
                });
            nrows += 1;
        });

    // We only only need to add obstacles along the path the
    // guard actually takes, which is a lot fewer than the
    // total number of open positions. The loop is slightly
    // different because if we placed an obstacle where the
    // guard is standing, she would see us.
    let start = guard;
    let mut dir = '^';
    let mut steps = HashSet::new();
    loop {
        (guard, dir) = step(guard, dir, &obstacles);
        if !is_inbounds(guard, nrows, ncols) {
            break;
        }
        steps.insert(guard);
    }

    steps.into_par_iter()
        .filter(|(r, c)| {
            let mut obs = obstacles.clone();
            if obs.insert((*r, *c)) {
                let mut dir = '^';
                let mut guard = start;
                let mut steps = HashSet::new();
                while is_inbounds(guard, nrows, ncols) {
                    if !steps.insert((guard, dir)) {
                        return true;
                    } else {
                        (guard, dir) = step(guard, dir, &obs);
                    }
                }
            }
            false
        })
        .count()
}

fn is_inbounds((row, col): (i32, i32), nrows: i32, ncols: i32) -> bool
{
    row > -1 && col > -1 && row < nrows && col < ncols
}

fn step((row, col): (i32, i32), dir: char, obstacles: &Obstacles) -> ((i32, i32), char)
{
    let mut pos = match dir {
        '^' => (row - 1, col),
        'v' => (row + 1, col),
        '<' => (row, col - 1),
        '>' => (row, col + 1),
         _  => unreachable!()
    };

    let mut d = dir;
    if obstacles.contains(&pos) {
        d = match dir {
            '^' => '>',
            'v' => '<',
            '<' => '^',
            '>' => 'v',
             _  => unreachable!()
        };
        pos = (row, col);
    }

    (pos, d)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 5153);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1711);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 41);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 6);
    }
}
