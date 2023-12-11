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
    let mut galaxies = vec![];

    input.lines()
        .enumerate()
        .for_each(|(row, line)| {
            cols = line.len() as i64;
            line.chars()
                .enumerate()
                .for_each(|(col, c)| if c == '#' {
                    galaxies.push(row  as i64 * cols + col as i64)
                });
        });
    
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
                .for_each(|pos| {
                    let r = *pos / cols;
                    if r > *row { *pos += (expansion - 1) * cols }
                });
            rows += expansion
        });

    expand_cols.iter().rev()
        .for_each(|col| {
            galaxies.iter_mut()
                .for_each(|pos| {
                    let c = *pos % cols;
                    *pos += (*pos / cols) * expansion;
                    if c > *col { *pos += expansion - 1 }
                });
            cols += expansion
        });

    galaxies.iter()
        .enumerate()
        .map(|(i, p)| (i, p / cols, p % cols))
        .map(|(i, ri, ci)|
            galaxies.iter()
                .skip(i + 1)
                .map(|n| (n / cols, n % cols))
                .map(|(rj, cj)| ri.abs_diff(rj) + ci.abs_diff(cj))
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
