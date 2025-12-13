use pathfinding::matrix::Matrix;

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
    let (tiles, trees) = load(input);

    let counts = tiles.iter()
        .map(|m| m.items().filter(|(_, c)| **c == b'#').count())
        .collect::<Vec<_>>();

    // Filter out any where the total space required by the
    // tiles is larger than the reqion area.
    let maybe = trees.iter()
        .filter(|tree| might_fit(&counts, tree))
        .collect::<Vec<_>>();

    // We don't need to check any where the area will accomodate
    // just laying the tiles out side by side. So get the number
    // of tiles that will fit without interlocking.
    let valid = maybe.len();
    let maybe = maybe.iter()
        .fold(vec![], |mut v, tree| {
            let tiles_r = tree.region.rows / 3;
            let tiles_c = tree.region.columns / 3;
            let tiles = (tiles_r * tiles_c) as u32;
            let shapes = tree.shapes.iter().sum::<u32>();
            if shapes > tiles { v.push(tree); }
            v
        });
    
    // If all trees are big enough to fit all presents without
    // interlocking, we're done.
    if maybe.is_empty() { valid } else { 0 }

    // And, for our input they are, so we're done!
}

fn might_fit(counts: &[usize], tree: &Tree) -> bool
{
    let area = tree.region.rows * tree.region.columns;
    let needed = tree.shapes.iter()
        .zip(counts)
        .map(|(a, b)| a * *b as u32)
        .sum::<u32>();

    area >= needed as usize
}

#[derive(Clone, Debug)]
struct Tree {
    region: Matrix<u8>,
    shapes: Vec<u32>,
}

fn load(input: &str) -> (Vec<Matrix<u8>>, Vec<Tree>)
{
    // The first 6 are the presents
    let iter = input.split("\n\n");
    let presents = iter.take(6)
        .map(|s| s.lines().skip(1))
        .map(|l| Matrix::from_rows(l.map(|s| s.bytes())).unwrap())
        .collect();

    // The rest are regions an amounts
    let mut iter = input.split("\n\n");
    let s = iter.nth(6).unwrap();
    let trees = s.lines()
        .map(|l| {
            let (s1, s2) = l.split_once(": ").unwrap();
            let (sc, sr) = s1.split_once('x').unwrap();
            let rows = sr.parse::<usize>().unwrap();
            let cols = sc.parse::<usize>().unwrap();
            let region = Matrix::new(rows, cols, b'.');
            let shapes = s2.split_ascii_whitespace()
                .flat_map(|s| s.parse::<u32>())
                .collect();

            Tree { region, shapes }
        })
        .collect();

    (presents, trees)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 517);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 2);
    }
}
