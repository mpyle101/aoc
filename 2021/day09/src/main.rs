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
    let (ncols, tubes) = load(input);
    tubes.iter()
        .enumerate()
        .filter(|&(i, h)| heights(i, &tubes, ncols).iter().all(|n| h < n))
        .map(|(_, h)| *h as u32 + 1)
        .sum()
}

fn part_two(input: &str) -> usize
{
    use std::collections::{HashSet, VecDeque};

    let (ncols, tubes) = load(input);
    let pts = tubes.iter()
        .enumerate()
        .filter(|&(i, h)| heights(i, &tubes, ncols).iter().all(|n| h < n))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let mut basins = pts.iter()
        .fold(vec![], |mut v, i| {
            let mut q = VecDeque::from([(*i, tubes[*i])]);
            let mut basin = HashSet::new();

            while let Some((p, h)) = q.pop_front() {
                if basin.insert(p) {
                    neighbors(p, &tubes, ncols).iter()
                        .filter(|(_, h1)| *h1 < 9 && *h1 > h )
                        .for_each(|(p1, h1)| q.push_back((*p1, *h1)))
                }
            }
            v.push(basin.len());
            v
        });

    basins.sort_by(|a, b| b.cmp(a));
    basins.iter().take(3).product()
}

fn heights(pos: usize, tubes: &[u8], ncols: usize) -> [u8;4]
{
    let (pos, ncols) = (pos as i32, ncols as i32);
    let (row, col)   = (pos / ncols, pos % ncols);
    let rows = 0..tubes.len() as i32 / ncols;
    let cols = 0..ncols;

    let mut hts = [u8::MAX;4];
    [(-1, 0), (0, -1), (0, 1), (1, 0)].iter()
        .enumerate()
        .map(|(i, (dr, dc))| (i, (row + dr, col + dc)))
        .filter(|(_, (r, c))| rows.contains(r) && cols.contains(c))
        .map(|(i, (r, c))| (i, r * ncols + c))
        .for_each(|(i, p)| hts[i] = tubes[p as usize]);

    hts
}

fn neighbors(pos: usize, tubes: &[u8], ncols: usize) -> Vec<(usize, u8)>
{
    let (pos, ncols) = (pos as i32, ncols as i32);
    let (row, col)   = (pos / ncols, pos % ncols);
    let rows = 0..tubes.len() as i32 / ncols;
    let cols = 0..ncols;

    [(-1, 0), (0, -1), (0, 1), (1, 0)].iter()
        .map(|(dr, dc)| (row + dr, col + dc))
        .filter(|(r, c)| rows.contains(r) && cols.contains(c))
        .map(|(r, c)| (r * ncols + c) as usize)
        .map(|p| (p, tubes[p]))
        .collect()
}

fn load(input: &str) -> (usize, Vec<u8>)
{
    let mut ncols = 0;
    let hts = input.lines()
        .map(|line| line.as_bytes())
        .map(|bytes| bytes.iter().map(|c| c - b'0'))
        .fold(vec![], |mut v, b| {
            ncols = b.len();
            v.extend(b);
            v
        });

    (ncols, hts)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 633);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1050192);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 15);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 1134);
    }

}