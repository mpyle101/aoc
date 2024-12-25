use std::collections::{BTreeSet, HashMap};

type Fence = ((i32, i32), (i32, i32));
type Fences = BTreeSet<Fence>;

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

fn part_one(input: &str) -> i32
{
    let mut ncols = 0;
    let mut nrows = 0;

    let garden = input.lines()
        .zip(0..)
        .fold(vec![], |mut v, (line, row)| {
            nrows = row + 1;
            ncols = line.len() as i32;
            v.extend(line.chars());
            v
        });
    let mut open = vec![true; garden.len()];

    (0..garden.len())
        .filter_map(|i| open[i].then_some(find_region(i as i32, nrows, ncols, &garden, &mut open)))
        .map(|region| region.len() as i32 * perimeter(nrows, ncols, region))
        .sum()
}

fn part_two(input: &str) -> i32
{
    let mut ncols = 0;
    let mut nrows = 0;

    let garden = input.lines()
        .zip(0..)
        .fold(vec![], |mut v, (line, row)| {
            nrows = row + 1;
            ncols = line.len() as i32;
            v.extend(line.chars());
            v
        });
    let mut open = vec![true; garden.len()];

    (0..garden.len())
        .filter_map(|i| open[i].then_some(find_fences(i as i32, nrows, ncols, &garden, &mut open)))
        .map(|(plots, fences)| plots * coalesce(fences))
        .sum()
}

fn find_region(pos: i32, nrows: i32, ncols: i32, garden: &[char], open: &mut [bool]) -> HashMap<(i32, i32), i32>
{
    use std::collections::VecDeque;

    let mut region = HashMap::new();

    let mut q = VecDeque::from([pos]);
    while let Some(pos) = q.pop_front() {
        let i = pos as usize;
        if open[i] {
            open[i] = false;
            let row = pos / ncols;
            let col = pos % ncols;

            region.insert((row, col), 4);
            let plant = garden[i];
            q.extend(neighbors((row, col)).iter()
                .filter(|p| is_inbounds(p, nrows, ncols))
                .map(|(row, col)| row * ncols + col)
                .filter(|&p| garden[p as usize] == plant && open[p as usize]))
        }
    }

    region
}

fn find_fences(pos: i32, nrows: i32, ncols: i32, garden: &[char], open: &mut [bool]) -> (i32, Fences)
{
    use std::collections::VecDeque;

    let mut plots = 0;
    let mut fences = Fences::new();

    // Each plot can be thought of as a set of unit vectors going around
    // the outside in one direction or another (we chose counter clockwise).
    // As new plots are considered, if a fence exists which is the exact
    // opposite of an existing fence, they cancel each other out so remove
    // the existing one and don't insert the new one. Otherwise, add the
    // new section to the list.
    // When finished, the resulting union of fences will be a set of unit
    // vectors making up the sections the external and internal sides. To
    // find the actual number of sides we then coalesce those sections into
    // a minimal set of larger vectors.
    let mut q = VecDeque::from([pos]);
    while let Some(pos) = q.pop_front() {
        let i = pos as usize;
        if open[i] {
            plots += 1;
            open[i] = false;
            let row = pos / ncols;
            let col = pos % ncols;
            for (p1, p2) in sections(row, col) {
                if !fences.remove(&(p2, p1)) {
                    fences.insert((p1, p2));
                }
            }

            let plant = garden[i];
            q.extend(neighbors((row, col)).iter()
                .filter(|p| is_inbounds(p, nrows, ncols))
                .map(|(row, col)| row * ncols + col)
                .filter(|&p| garden[p as usize] == plant && open[p as usize]))
        }
    }

    (plots, fences)
}

fn perimeter(nrows: i32, ncols: i32, mut region: HashMap<(i32, i32), i32>) -> i32
{
    let keys: Vec<_> = region.keys().cloned().collect();
    for pos in keys {
        let count = neighbors(pos).iter()
            .filter(|p| is_inbounds(p, nrows, ncols))
            .filter(|p| region.contains_key(p))
            .count() as i32;
        region.entry(pos).and_modify(|n| *n -= count);
    }

    region.values().sum()
}

fn coalesce(mut fences: Fences) -> i32
{
    // While there are fence sections left, grab the next one
    // and attempt to grow it as much as possible in both directions
    // When we're out of sections, we'll have the number of
    // contiguous sides.
    let mut sides = 0;
    while let Some(mut v) = fences.pop_first() {
        sides += 1;
        let dr = v.1.0 - v.0.0;
        let dc = v.1.1 - v.0.1;

        let mut growing = true;
        while growing {
            let v1 = ((v.0.0 - dr, v.0.1 - dc), v.0);
            growing = fences.remove(&v1);
            if growing {
                v = (v1.0, v.1)
            } 

            let v1 = (v.1, (v.1.0 + dr, v.1.1 + dc));
            if fences.remove(&v1) {
                v = (v.0, v1.1);
                growing = true;
            }
        }
    }

    sides
}

fn is_inbounds((row, col): &(i32, i32), nrows: i32, ncols: i32) -> bool
{
    *row >= 0 && *col >= 0 && *row < nrows && *col < ncols
}

fn neighbors((row, col): (i32, i32)) -> [(i32, i32);4]
{
    [(row, col + 1), (row + 1, col), (row - 1, col), (row, col - 1)]
}

fn sections(row: i32, col: i32) -> [Fence; 4]
{
    [
        ((row, col), (row + 1, col)),
        ((row + 1, col), (row + 1, col + 1)),
        ((row + 1, col + 1), (row, col + 1)),
        ((row, col + 1), (row, col)),
    ]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1381056);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 834828);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example4.txt");
        assert_eq!(part_one(input), 1930);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_two(input), 80);

        let input = include_str!("../example2.txt");
        assert_eq!(part_two(input), 236);

        let input = include_str!("../example3.txt");
        assert_eq!(part_two(input), 368);

        let input = include_str!("../example4.txt");
        assert_eq!(part_two(input), 1206);
    }
}
