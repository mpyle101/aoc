use pathfinding::matrix::Matrix;

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
    let disks = input.lines()
        .skip(2)
        .map(|s| {
            let size = s[24..27].trim().parse::<u32>().unwrap();
            let used = s[30..33].trim().parse::<u32>().unwrap();
            (size, used)
        })
        .collect::<Vec<_>>();

    let mut viable = 0;
    for a in 0..disks.len() - 1 {
        for b in a + 1..disks.len() {
            if disks[a].1 > 0 {
                viable += (disks[a].1 <= disks[b].0 - disks[b].1) as u32
            }
            if disks[b].1 > 0 {
                viable += (disks[b].1 <= disks[a].0 - disks[a].1) as u32
            }
        }
    }

    viable
}

fn part_two(input: &str) -> u32
{
    use pathfinding::prelude::dijkstra;

    let mut empty = (0, 0);
    let mut disks = Matrix::new(29, 35, (0, 0));
    input.lines()
        .skip(2)
        .for_each(|s| {
            let (sx, sy) = &s[15..23].split_once('-').unwrap();
            let c = sx[1..].parse::<usize>().unwrap();
            let r = sy[1..].trim().parse::<usize>().unwrap();

            let size = s[24..27].trim().parse::<u32>().unwrap();
            let used = s[30..33].trim().parse::<u32>().unwrap();
            if used == 0 { empty = (r, c) }

            disks[(r, c)] = (size, used);
        });

    // Drawing shows there's only one disk with enough available
    // space to hold the target data. There's also a "wall" of
    // huge disks between the empty disk and the target disk.
    //
    // . . . . . G
    // . # # # # #
    // . . . _ . .

    // This gets us a path to be next to G
    let goal = (0, 33);
    let avail = disks[empty].0;
    let path = dijkstra(
        &empty,
        |p| successors(p, &disks, avail).into_iter().map(|i| (i, 1)),
        |p| *p == goal
    ).unwrap();

    // It take 5 data moves to move G left 1 disk and move the empty
    // disk to the left of the new G. So 4 times the number of disks
    // to move G to column 1 and one last move to put it at (0, 0).
    path.1 + 5 * 33 + 1
}

fn successors(pos: &(usize, usize), disks: &Matrix<(u32, u32)>, avail: u32) -> Vec<(usize, usize)>
{
    disks.neighbours(*pos, false)
        .filter(|p| disks[p].1 <= avail)
        .collect()
}

#[allow(dead_code)]
fn draw(disks: &Matrix<(u32, u32)>, path: &[(usize, usize)])
{
    (0..29).for_each(|r| {
        (0..35).for_each(|c| {
            let disk = disks[(r, c)];
            if r == 0 && c == 0 {
                print!("(.)")
            } else if c == 34 && r == 0 {
                print!(" G ")
            } else if path.contains(&(r, c)) {
                print!(" * ")
            } else if disk.1 == 0 {
                print!(" - ")
            } else if disk.0 > 500 {
                print!(" # ")
            } else {
                print!(" . ")
            }
        });
        println!();
    });
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1003);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 192);
    }
}
