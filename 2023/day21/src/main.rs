use std::collections::{HashSet, HashMap};

type Tiles = HashSet<i32>;
type Farm = HashMap<(i32, i32), Tiles>;

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
    let (nrows, ncols, start, rocks) = load(input);
    march(64, nrows, ncols, start, &rocks)
}

fn part_two(input: &str) -> usize
{
    let (nrows, ncols, start, rocks) = load(input);
    sprint(26_501_365, nrows, ncols, start, &rocks)
}

#[allow(dead_code)]
fn print(nrows: i32, ncols: i32, tiles: &Tiles, rocks: &Tiles)
{
    println!("{:?}", tiles);
    for row in 0..nrows {
        for col in 0..ncols {
            let pos = row * ncols + col;
            if rocks.contains(&pos) {
                print!("#")
            } else if tiles.contains(&pos) {
                print!("O")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

fn load(input: &str) -> (i32, i32, i32, Tiles)
{
    let mut nrows = 0;
    let mut ncols = 0;
    let mut start = 0;
    let rocks: HashSet<_> = input.lines()
        .zip(0..)
        .flat_map(|(line, row)| {
            nrows = row + 1;
            ncols = line.len() as i32;
            line.chars()
                .zip(0..)
                .inspect(|(c, col)| if *c == 'S' { start = row * ncols + col; })
                .filter(|(c, _)| *c == '#')
                .map(|(_, col)| row * ncols + col)
                .collect::<Vec<_>>()
        })
        .collect();

    (nrows, ncols, start, rocks)
}

fn march(steps: i32, nrows: i32, ncols: i32, start: i32, rocks: &Tiles) -> usize
{
    let mut tiles = HashSet::from([start]);
    (0..steps).for_each(|_| tiles = step(nrows, ncols, &tiles, rocks));
    tiles.len()
}

fn step(nrows: i32, ncols: i32, tiles: &Tiles, rocks: &Tiles) -> Tiles
{
    let mut steps = HashSet::new();

    tiles.iter()
        .for_each(|pos| {
            let row = pos / ncols;
            let col = pos % ncols;
            if row > 0 { steps.insert(pos - ncols); }
            if col > 0 { steps.insert(pos - 1); }
            if row < nrows - 1 { steps.insert(pos + ncols); }
            if col < ncols - 1 { steps.insert(pos + 1); }
        });

    &steps - rocks
}

fn sprint(steps: i32, nrows: i32, ncols: i32, start: i32, rocks: &Tiles) -> usize
{
    let mut farm = Farm::from([((0, 0), Tiles::from([start]))]);
    for _ in 0..steps { farm = stride(nrows, ncols, &farm, rocks);};

    farm.values().map(|v| v.len()).sum()
}

fn stride(nrows: i32, ncols: i32, farm: &Farm, rocks: &Tiles) -> Farm
{
    let mut acres = Farm::new();

    for ((r, c), tiles) in farm {
        tiles.iter()
            .for_each(|pos| {
                let row = pos / ncols;
                let col = pos % ncols;

                // Step up
                if row == 0 {
                    let p = (nrows - 1) * ncols + col;
                    acres.entry((r - 1, *c)).or_default().insert(p);
                } else {
                    let p = pos - ncols;
                    acres.entry((*r, *c)).or_default().insert(p);
                }

                // Step left
                if col == 0 { 
                    let p = row * ncols + ncols - 1;
                    acres.entry((*r, c - 1)).or_default().insert(p);
                } else {
                    let p = pos - 1;
                    acres.entry((*r, *c)).or_default().insert(p);
                }

                // Step down
                if row == nrows - 1 {
                    let p = col;
                    acres.entry((r + 1, *c)).or_default().insert(p);
                } else {
                    let p = pos + ncols;
                    acres.entry((*r, *c)).or_default().insert(p);
                }

                // Step right
                if col == ncols - 1 {
                    let p = row * ncols;
                    acres.entry((*r, c + 1)).or_default().insert(p);
                } else { 
                    let p = pos + 1;
                    acres.entry((*r, *c)).or_default().insert(p);
                }
            });
    }

    acres.iter()
        .map(|(p, tiles)| (*p, tiles - rocks))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 3585);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        let (nrows, ncols, start, rocks) = load(input);
        let tiles = march(6, nrows, ncols, start, &rocks);

        assert_eq!(tiles, 16);
    }

    #[test]
    fn example_part_two_50()
    {
        let input = include_str!("../example.txt");
        let (nrows, ncols, start, rocks) = load(input);
        let tiles = sprint(50, nrows, ncols, start, &rocks);

        assert_eq!(tiles, 1594);
    }

    #[test]
    fn example_part_two_100()
    {
        let input = include_str!("../example.txt");
        let (nrows, ncols, start, rocks) = load(input);
        let tiles = sprint(100, nrows, ncols, start, &rocks);

        assert_eq!(tiles, 6536);
    }
}
