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
    let mut field = load(input);
    tilt_north(&mut field);

    let nrows = field.len();
    field.into_iter()
        .enumerate()
        .map(|(row, v)| v.into_iter()
            .filter(|c| *c == 'O')
            .count() * (nrows - row)
        )
        .sum()
}

fn part_two(input: &str) -> usize
{
    use std::collections::HashMap;

    let mut field = load(input);

    let mut left = 0;
    let mut states = HashMap::new();
    for cycle in 1..=1_000_000_000 {
        tilt_north(&mut field);
        tilt_west(&mut field);
        tilt_south(&mut field);
        tilt_east(&mut field);
    
        if let Some(n) = states.get(&field) {
            let step = cycle - n;
            left = (1_000_000_000 - cycle) % step;
            break;
        } else {
            states.insert(field.clone(), cycle);
        }
    }

    for _ in 0..left {
        tilt_north(&mut field);
        tilt_west(&mut field);
        tilt_south(&mut field);
        tilt_east(&mut field);
    }

    let nrows = field.len();
    field.into_iter()
        .enumerate()
        .map(|(row, v)| v.into_iter()
            .filter(|c| *c == 'O')
            .count() * (nrows - row)
        )
        .sum()
}

fn load(input: &str) -> Vec<Vec<char>>
{
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn tilt_north(field: &mut [Vec<char>])
{
    let ncols = field[0].len();

    for row in 1..field.len() {
        for c in 0..ncols {
            if field[row][c] == 'O' {
                for r in (0..row).rev() {
                    if field[r][c] != '.' { break; }
                    field[r+1][c] = '.';
                    field[r][c] = 'O'
                }
            }
        }
    }
}

fn tilt_south(field: &mut [Vec<char>])
{
    let nrows = field.len();
    let ncols = field[0].len();

    for row in (0..nrows).rev() {
        for c in 0..ncols {
            if field[row][c] == 'O' {
                for r in row+1..nrows {
                    if field[r][c] != '.' { break; }
                    field[r-1][c] = '.';
                    field[r][c] = 'O'
                }
            }
        }
    }
}

fn tilt_west(field: &mut [Vec<char>])
{
    let ncols = field[0].len();

    for row in field.iter_mut() {
        for col in 1..ncols {
            if row[col] == 'O' {
                for c in (0..col).rev() {
                    if row[c] != '.' { break; }
                    row[c+1] = '.';
                    row[c] = 'O'
                }
            }
        }
    }
}

fn tilt_east(field: &mut [Vec<char>])
{
    let ncols = field[0].len();

    for row in field.iter_mut() {
        for col in (0..ncols).rev() {
            if row[col] == 'O' {
                for c in col+1..ncols {
                    if row[c] != '.' { break; }
                    row[c-1] = '.';
                    row[c] = 'O'
                }
            }
        }
    }
}

#[allow(dead_code)]
fn print_field(field: &[Vec<char>])
{
    for row in field {
        for c in row {
            print!("{c}")
        }
        println!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 102497);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 105008);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 136);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 64);
    }
}
