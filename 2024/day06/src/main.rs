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
    use std::iter::from_fn;

    let mut ncols = 0;
    let mut nrows = 0;
    let mut start = (-1, -1);

    let obstacles = input.lines()
        .zip(0..)
        .fold(Obstacles::new(), |mut m, (line, row)| {
            nrows = row + 1;
            ncols = line.len() as i32;
            line.chars()
                .zip(0..)
                .filter(|(c, _)| *c != '.')
                .for_each(|(c, col)| {
                    if c == '#' {
                        m.insert((row, col));
                    } else {
                        start = (row, col);
                    }
                });
            m
        });

    let mut dir = '^';
    let mut guard = start;
    let steps = from_fn(|| {
        (guard, dir) = step(guard, dir, &obstacles);
        is_inbounds(guard, nrows, ncols).then_some(guard)
    })
    .collect::<HashSet<_>>();

    // If the starting location isn't in the set add 1
    steps.len() + !steps.contains(&start) as usize
}

fn part_two(input: &str) -> usize
{
    use std::iter::from_fn;
    use rayon::prelude::*;

    let mut ncols = 0;
    let mut nrows = 0;
    let mut start = (-1, -1);

    let obstacles = input.lines()
        .zip(0..)
        .fold(Obstacles::new(), |mut m, (line, row)| {
            nrows = row + 1;
            ncols = line.len() as i32;
            line.chars()
                .zip(0..)
                .filter(|(c, _)| *c != '.')
                .for_each(|(c, col)| {
                    if c == '#' {
                        m.insert((row, col));
                    } else {
                        start = (row, col);
                    }
                });
            m
        });

    // We only only need to add obstacles along the path the
    // guard actually takes, which is a lot fewer than the
    // total number of open positions. Don't worry about the
    // guard's starting location because if we placed an
    // obstacle there, she would see us.
    let mut dir = '^';
    let mut guard = start;
    let steps: HashSet<_> = from_fn(|| {
        (guard, dir) = step(guard, dir, &obstacles);
        is_inbounds(guard, nrows, ncols).then_some(guard)
    })
    .collect();

    steps.into_par_iter()
        .map(|p| {
            let mut obs = obstacles.clone();
            obs.insert(p); obs
        })
        .filter(|obs| {
            let mut dir = '^';
            let mut guard = start;
            let mut steps = HashSet::new();
            while is_inbounds(guard, nrows, ncols) {
                if !steps.insert((guard, dir)) {
                    return true;
                } else {
                    (guard, dir) = step(guard, dir, obs);
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
    let (next, turn) = match dir {
        '^' => ((row - 1, col), '>'),
        'v' => ((row + 1, col), '<'),
        '<' => ((row, col - 1), '^'),
        '>' => ((row, col + 1), 'v'),
         _  => unreachable!()
    };

    if obstacles.contains(&next) {
        ((row, col), turn)
    } else {
        (next, dir)
    }
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
