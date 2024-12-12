use std::collections::HashMap;

type Fence = ((usize, usize), (usize, usize));
type Fences = Vec<(Fence, (i32, i32))>;

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
    let mut ncols = 0;
    let mut nrows = 0;

    let garden = input.lines()
        .enumerate()
        .fold(vec![], |mut v, (row, line)| {
            nrows = row + 1;
            ncols = line.len();
            v.extend(line.chars());
            v
        });
    let mut open = vec![true; garden.len()];

    (0..garden.len())
        .filter_map(|i| open[i].then_some(find_region(i, nrows, ncols, &garden, &mut open)))
        .map(|region| region.len() * perimeter(nrows, ncols, region))
        .sum()
}

fn part_two(input: &str) -> usize
{
    let mut ncols = 0;
    let mut nrows = 0;

    let garden = input.lines()
        .enumerate()
        .fold(vec![], |mut v, (row, line)| {
            nrows = row + 1;
            ncols = line.len();
            v.extend(line.chars());
            v
        });
    let mut open = vec![true; garden.len()];

    (0..garden.len())
        .filter_map(|i| open[i].then_some(find_fences(i, nrows, ncols, &garden, &mut open)))
        .map(|(plots, fences)| plots * coalesce(fences))
        .sum()
}

fn find_region(pos: usize, nrows: usize, ncols: usize, garden: &[char], open: &mut [bool]) -> HashMap<(usize, usize), usize>
{
    use std::collections::VecDeque;

    let mut region = HashMap::new();

    let mut q = VecDeque::from([pos]);
    while let Some(pos) = q.pop_front() {
        if open[pos] {
            open[pos] = false;
            let row = pos / ncols;
            let col = pos % ncols;

            region.insert((row, col), 4);
            let plant = garden[pos];
            q.extend(neighbors((row, col), nrows, ncols).iter()
                .map(|(row, col)| row * ncols + col)
                .filter(|&p| garden[p] == plant && open[p]))
        }
    }

    region
}

fn find_fences(pos: usize, nrows: usize, ncols: usize, garden: &[char], open: &mut [bool]) -> (usize, Fences)
{
    use std::collections::{HashSet, VecDeque};

    let mut plots = 0;
    let mut fences = HashSet::new();

    let mut q = VecDeque::from([pos]);
    while let Some(pos) = q.pop_front() {
        if open[pos] {
            plots += 1;
            open[pos] = false;
            let row = pos / ncols;
            let col = pos % ncols;
            [
                ((row, col), (row + 1, col)),
                ((row + 1, col), (row + 1, col + 1)),
                ((row + 1, col + 1), (row, col + 1)),
                ((row, col + 1), (row, col)),
            ].iter()
                .for_each(|&(p1, p2)| 
                    if fences.contains(&(p2, p1)) {
                        fences.remove(&(p2, p1));
                    } else {
                        fences.insert((p1, p2));
                    }
                );

            let plant = garden[pos];
            q.extend(neighbors((row, col), nrows, ncols).iter()
                .map(|(row, col)| row * ncols + col)
                .filter(|&p| garden[p] == plant && open[p]))
        }
    }
    let fences = fences.iter()
        .map(|p| {
            let dr = p.0.0 as i32 - p.1.0 as i32;
            let dc = p.0.1 as i32 - p.1.1 as i32;
            (*p, (dr.signum(), dc.signum()))
        })
        .collect();

    (plots, fences)
}

fn perimeter(nrows: usize, ncols: usize, mut region: HashMap<(usize, usize), usize>) -> usize
{
    let keys: Vec<_> = region.keys().cloned().collect();
    for pos in keys {
        let count = neighbors(pos, nrows, ncols).iter()
            .filter(|p| region.contains_key(p))
            .count();
        region.entry(pos).and_modify(|n| *n -= count);
    }

    region.values().sum()
}

fn coalesce(mut fences: Fences) -> usize
{
    let mut sides = 0;
    while let Some((mut v1, d1)) = fences.pop() {
        sides += 1;

        let mut growing: bool;
        loop {
            if let Some(i) = fences.iter()
                .position(|&(v2, d2)| d2 == d1 && v2.1 == v1.0)
            {
                let (v2, _) = fences.remove(i);
                v1 = (v2.0, v1.1);
                growing = true;
            } else {
                growing = false;
            }
            if let Some(i) = fences.iter()
                .position(|&(v2, d2)| d2 == d1 && v1.1 == v2.0)
            {
                let (v2, _) = fences.remove(i);
                v1 = (v1.0, v2.1);
                growing = true;
            }
            //println!("{growing} {:?} {:?}", v1, fences.len());

            if !growing { break; }
        }
    }

    sides
}

fn neighbors((row, col): (usize, usize), nrows: usize, ncols: usize) -> Vec<(usize, usize)>
{
    let mut v = vec![];
    if col < ncols - 1 {
        v.push((row, col + 1));
    }
    if row < nrows - 1 {
        v.push((row + 1, col))
    }
    if row > 0 {
        v.push((row - 1, col));
    }
    if col > 0 {
        v.push((row, col - 1));
    }

    v
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
