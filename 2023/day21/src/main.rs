use std::collections::HashSet;

type Tiles = HashSet<i32>;

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
    let (nrows, ncols, mut tiles, rocks) = load(input);
    (0..64).for_each(|_| tiles = step(nrows, ncols, &tiles, &rocks));

    tiles.len()
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

fn load(input: &str) -> (i32, i32, Tiles, Tiles)
{
    let mut nrows = 0;
    let mut ncols = 0;
    let mut tiles = HashSet::new();
    let rocks: HashSet<_> = input.lines()
        .zip(0..)
        .flat_map(|(line, row)| {
            nrows = row + 1;
            ncols = line.len() as i32;
            line.chars()
                .zip(0..)
                .inspect(|(c, col)| if *c == 'S' { tiles.insert(row * ncols + col); })
                .filter(|(c, _)| *c == '#')
                .map(|(_, col)| row * ncols + col)
                .collect::<Vec<_>>()
        })
        .collect();

    (nrows, ncols, tiles, rocks)
}

fn step(nrows: i32, ncols: i32, tiles: &Tiles, rocks: &Tiles) -> Tiles
{
    let mut new = HashSet::new();

    tiles.iter()
        .for_each(|pos| {
            let row = pos / ncols;
            let col = pos % ncols;
            if row > 0 { new.insert(pos - ncols); }
            if col > 0 { new.insert(pos - 1); }
            if row < nrows - 1 { new.insert(pos + ncols); }
            if col < ncols - 1 { new.insert(pos + 1); }
        });

    &new - rocks
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
        let (nrows, ncols, mut tiles, rocks) = load(input);
        (0..6).for_each(|_| tiles = step(nrows, ncols, &tiles, &rocks));
    
        assert_eq!(tiles.len(), 16);
    }
}
