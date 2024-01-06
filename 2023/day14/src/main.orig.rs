use std::collections::HashSet;

type Rocks = HashSet<u32>;

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

fn part_one(input: &str) -> u32
{
    let (ncols, mut stones, rocks) = load(input);
    let nrows = rocks.iter().max().unwrap() / ncols + 1;
    stones = tilt_north(ncols, &stones, &rocks);
    stones.iter()
        .map(|p| nrows - p / ncols)
        .sum()
}

fn part_two(input: &str) -> u32
{
    use std::collections::HashMap;

    let (ncols, mut stones, rocks) = load(input);
    let nrows = rocks.iter().max().unwrap() / ncols + 1;

    let mut left = 0;
    let mut states = HashMap::new();
    for cycle in 1..=1_000_000_000 {
        stones = tilt_north(ncols, &stones, &rocks);    
        stones = tilt_west(ncols, &stones, &rocks);
        stones = tilt_south(ncols, nrows, &stones, &rocks);
        stones = tilt_east(ncols, &stones, &rocks);
    
        if let Some(n) = states.get(&stones) {
            let step = cycle - n;
            left = (1_000_000_000 - cycle) % step;
            break;
        } else {
            states.insert(stones.clone(), cycle);
        }
    }

    for _ in 0..left {
        stones = tilt_north(ncols, &stones, &rocks);    
        stones = tilt_west(ncols, &stones, &rocks);
        stones = tilt_south(ncols, nrows, &stones, &rocks);
        stones = tilt_east(ncols, &stones, &rocks);
    }

    stones.iter()
        .map(|p| nrows - p / ncols)
        .sum()
}

#[allow(dead_code)]
fn print_field(ncols: u32, nrows: u32, stones: &[u32], rocks: &Rocks)
{
    for row in 0..nrows {
        for col in 0..ncols {
            let p = row * ncols + col;
            if stones.contains(&p) {
                print!("O")
            } else if rocks.contains(&p) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

fn load(input: &str) -> (u32, Vec<u32>, Rocks)
{
    let mut stones = vec![];
    let mut rocks = HashSet::new();

    let mut ncols = 0;
    input.lines()
        .zip(0..)
        .for_each(|(line, row)| {
            ncols = line.len() as u32;
            line.chars()
                .zip(0..)
                .for_each(|(c, col)| {
                    let pos = row * ncols + col;
                    if c == 'O' {
                        stones.push(pos);
                    } else if c == '#' {
                        rocks.insert(pos);
                    }
                })
        });

    (ncols, stones, rocks)
}

fn tilt_north(ncols: u32, stones: &[u32], rocks: &Rocks) -> Vec<u32>
{
    let mut v = Vec::with_capacity(stones.len());

    stones.iter()
        .for_each(|p| {
            let row = p / ncols;
            let col = p % ncols;

            let mut row1 = row;
            for r in (0..row).rev() {
                let p = r * ncols + col;
                if v.contains(&p) || rocks.contains(&p) {
                    break;
                }

                row1 = r
            }
            v.push(row1 * ncols + col);
        });

    v.sort();
    v
}

fn tilt_west(ncols: u32, stones: &[u32], rocks: &Rocks) -> Vec<u32>
{
    let mut v = Vec::with_capacity(stones.len());

    stones.iter()
        .for_each(|p| {
            let row = p / ncols;
            let col = p % ncols;

            if col == 0 {
                v.push(*p)
            } else {
                let mut col1 = col;
                for c in (0..col).rev() {
                    let p = row * ncols + c;
                    if v.contains(&p) || rocks.contains(&p) {
                        break;
                    }
    
                    col1 = c
                }
                v.push(row * ncols + col1);
            }
        });

    v.sort();
    v
}

fn tilt_south(ncols: u32, nrows: u32, stones: &[u32], rocks: &Rocks) -> Vec<u32>
{
    let mut v = Vec::with_capacity(stones.len());

    stones.iter().rev()
        .for_each(|p| {
            let row = p / ncols;
            let col = p % ncols;

            let mut row1 = row;
            for r in row..nrows {
                let p = r * ncols + col;
                if v.contains(&p) || rocks.contains(&p) {
                    break;
                }

                row1 = r
            }
            v.push(row1 * ncols + col);
        });

    v.sort();
    v
}

fn tilt_east(ncols: u32, stones: &[u32], rocks: &Rocks) -> Vec<u32>
{
    let mut v = Vec::with_capacity(stones.len());

    stones.iter().rev()
        .for_each(|p| {
            let row = p / ncols;
            let col = p % ncols;

            let mut col1 = col;
            for c in col+1..ncols {
                let p = row * ncols + c;
                if v.contains(&p) || rocks.contains(&p) {
                    break;
                }

                col1 = c
            }
            v.push(row * ncols + col1);
        });

    v.sort();
    v
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
