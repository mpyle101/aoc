fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input, 1000000);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    let mut cols = 0;
    let mut galaxies = vec![];

    let mut row = 0;
    input.lines()
        .for_each(|line| {
            cols = line.len() as i32;
            let size = galaxies.len();
            line.chars()
                .enumerate()
                .for_each(|(col, c)| if c == '#' {
                    galaxies.push(row * cols + col as i32)
                });
            if galaxies.len() == size {
                row += 2
            } else {
                row += 1
            }
        });
    
    let rows = galaxies.last().unwrap() / cols + 1;
    let mut expand = vec![];
    (0..cols)
        .for_each(|col| {
            let mut empty = true;
            for row in 0..rows {
                let pos = row * cols + col;
                if galaxies.contains(&pos) {
                    empty = false;
                    break;
                }
            }
            if empty {
                expand.push(col)
            }
        });

    expand.iter().rev()
        .for_each(|col| {
            for pos in galaxies.iter_mut() {
                let c = *pos % cols;
                *pos += *pos / cols;
                if c > *col { *pos += 1 }
            }
            cols += 1
        });

    let mut paths = 0;
    for i in 0..galaxies.len() {
        let pi = galaxies[i];
        let ri = pi / cols;
        let ci = pi % cols;

        galaxies.iter()
            .skip(i + 1)
            .for_each(|n| {
                let rj = n / cols;
                let cj = n % cols;
                let md = ri.abs_diff(rj) + ci.abs_diff(cj);

                paths += md
            })
    }

    paths
}

fn part_two(input: &str, expansion: i64) -> u64
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

    let mut expand_rows = vec![];
    (0..rows)
        .for_each(|row| {
            let mut empty = true;
            for col in 0..cols {
                let pos = row * cols + col;
                if galaxies.contains(&pos) {
                    empty = false;
                    break;
                }
            }
            if empty {
                expand_rows.push(row)
            }
        });

    let mut expand_cols = vec![];
    (0..cols)
        .for_each(|col| {
            let mut empty = true;
            for row in 0..rows {
                let pos = row * cols + col;
                if galaxies.contains(&pos) {
                    empty = false;
                    break;
                }
            }
            if empty {
                expand_cols.push(col)
            }
        });

    expand_rows.iter().rev()
        .for_each(|row| {
            for pos in galaxies.iter_mut() {
                let r = *pos / cols;
                if r > *row { *pos += (expansion - 1) * cols }
            }
            rows += expansion
        });

    expand_cols.iter().rev()
        .for_each(|col| {
            for pos in galaxies.iter_mut() {
                let c = *pos % cols;
                *pos += (*pos / cols) * expansion;
                if c > *col { *pos += expansion - 1 }
            }
            cols += expansion
        });

    let mut paths = 0;
    for i in 0..galaxies.len() {
        let pi = galaxies[i];
        let ri = pi / cols;
        let ci = pi % cols;

        galaxies.iter()
            .skip(i + 1)
            .for_each(|n| {
                let rj = n / cols;
                let cj = n % cols;
                let md = ri.abs_diff(rj) + ci.abs_diff(cj);

                paths += md
            })
    }

    paths
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
        assert_eq!(part_two(input, 1000000), 904633799472);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 374);
    }

    #[test]
    fn example_part_two_2()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input, 2), 374);
    }

    #[test]
    fn example_part_two_10()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input, 10), 1030);
    }

    #[test]
    fn example_part_two_100()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input, 100), 8410);
    }
}
