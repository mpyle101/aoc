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

fn part_one(input: &str) -> u64
{
    sum_paths(input, 2)
}

fn part_two(input: &str) -> u64
{
    sum_paths(input, 1000000)
}

fn sum_paths(input: &str, expansion: i64) -> u64
{
    let mut cols = 0;
    let mut galaxies = input.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            cols = line.len() as i64;
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(col, _)| row as i64 * cols + col as i64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    
    let mut rows = galaxies.last().unwrap() / cols + 1;

    let expand_rows = (0..rows)
        .filter(|row| {
            ! (0..cols).any(|col| {
                let pos = row * cols + col;
                galaxies.contains(&pos)
            }) 
        })
        .collect::<Vec<_>>();

    let expand_cols = (0..cols)
        .filter(|col| {
            ! (0..rows).any(|row| {
                let pos = row * cols + col;
                galaxies.contains(&pos)
            }) 
        })
        .collect::<Vec<_>>();

    expand_rows.iter().rev()
        .for_each(|row| {
            galaxies.iter_mut()
                .map(|pos| (*pos / cols, pos))
                .filter(|(r, _)| r > row)
                .for_each(|(_, pos)| { *pos += (expansion - 1) * cols; });
            rows += expansion;
        });

    expand_cols.iter().rev()
        .for_each(|col| {
            galaxies.iter_mut()
                .for_each(|pos| {
                    let c = *pos % cols;
                    *pos += (*pos / cols) * expansion;
                    if c > *col { *pos += expansion - 1 }
                });
            cols += expansion;
        });

    // Sum the manhattan distances for each unique pair of galaxies.
    // (1, 2), (1, 3)...(1, n), (2, 3), (2, 4)...(2, n), etc.
    galaxies.iter()
        .enumerate()
        .map(|(i, pos)| (i, pos % cols, pos / cols))
        .map(|(i, x1, y1)|
            galaxies.iter()
                .skip(i + 1)
                .map(|pos| (pos % cols, pos / cols))
                .map(|(x2, y2)| x1.abs_diff(x2) + y1.abs_diff(y2))
                .sum::<u64>()
        )
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 9608724);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 904633799472);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 374);
    }

    #[test]
    fn sum_paths_2()
    {
        let input = include_str!("../example.txt");
        assert_eq!(sum_paths(input, 2), 374);
    }

    #[test]
    fn sum_paths_10()
    {
        let input = include_str!("../example.txt");
        assert_eq!(sum_paths(input, 10), 1030);
    }

    #[test]
    fn sum_paths100()
    {
        let input = include_str!("../example.txt");
        assert_eq!(sum_paths(input, 100), 8410);
    }
}
