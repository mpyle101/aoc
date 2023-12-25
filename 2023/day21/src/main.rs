#![allow(dead_code)]

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
    teleport(26_501_365, nrows, ncols, start, &rocks)
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

fn teleport(steps: usize, nrows: i32, ncols: i32, start: i32, rocks: &Tiles) -> usize
{
    let mut corners = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut fill = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    let mut farm = Farm::from([((0, 0), Tiles::from([start]))]);
    for i in 1..=391 {
        farm = stride(nrows, ncols, &farm, rocks);
        if (66..=260).contains(&i) {
            corners[0].push(farm.get(&(-1,  0)).unwrap().len());
            corners[1].push(farm.get(&( 0,  1)).unwrap().len());
            corners[2].push(farm.get(&( 1,  0)).unwrap().len());
            corners[3].push(farm.get(&( 0, -1)).unwrap().len());
        }
        if (132..=391).contains(&i) {
            fill[0].push(farm.get(&(-1, -1)).unwrap().len());
            fill[1].push(farm.get(&(-1,  1)).unwrap().len());
            fill[2].push(farm.get(&( 1, -1)).unwrap().len());
            fill[3].push(farm.get(&( 1,  1)).unwrap().len());
        }
    }

    let m = (steps - 260) % 131;

    let n1 = (steps - 260) / 131;
    let n2 = (steps - 66) % 131;
    let mut s2 = if m % 2 == 0 {
        (n1 + 2).pow(2) * 7265 + (n1 + 1).pow(2) * 7325
    } else {
        (n1 + 2).pow(2) * 7325 + (n1 + 1).pow(2) * 7265
    };
    
    s2 += corners.iter().map(|v| v[n2]).sum::<usize>();
    if m > 2 {
        let f1 = m - 3;
        let f2 = f1 + 131;
        s2 += fill.iter().map(|v| v[f1] * (n1 + 2)).sum::<usize>();
        s2 += fill.iter().map(|v| v[f2] * (n1 + 1)).sum::<usize>();
    }

    s2
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
    for _ in 1..steps { farm = stride(nrows, ncols, &farm, rocks); };
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
        .filter(|(_, tiles)| !tiles.is_empty())
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
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 597102953699891);
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
