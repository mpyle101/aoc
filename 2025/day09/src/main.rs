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

#[derive(Clone, Copy, Debug)]
struct Tile { x: u64, y: u64 }

type Edge = (u64, u64, u64);

fn part_one(input: &str) -> u64
{
    let tiles = input.lines()
        .map(|s| {
            let (s1, s2) = s.split_once(',').unwrap();
            let x = s1.parse::<u64>().unwrap();
            let y = s2.parse::<u64>().unwrap();

            Tile { x, y }
        })
        .collect::<Vec<_>>();

    let mut area = 0;
    for i in 0..tiles.len() - 1 {
        let a = &tiles[i];
        for b in tiles.iter().skip(i) {
            let d = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
            area = area.max(d);
        }
    }

    area
}

fn part_two(input: &str) -> u64
{
    let tiles = input.lines()
        .map(|s| {
            let (s1, s2) = s.split_once(',').unwrap();
            let x = s1.parse::<u64>().unwrap();
            let y = s2.parse::<u64>().unwrap();

            Tile { x, y }
        })
        .collect::<Vec<_>>();

    let (mut h_edges, mut v_edges) = tiles.iter()
        .enumerate()
        .fold((vec![], vec![]), |(mut h, mut v), (i, a)| {
            let b = &tiles[(i + 1) % tiles.len()];
            if a.x == b.x {
                v.push((a.x, a.y.min(b.y), a.y.max(b.y)))
            } else {
                h.push((a.y, a.x.min(b.x), a.x.max(b.x)))
            }
            (h, v)
        });
    h_edges.sort();
    v_edges.sort();

    let mut rects = vec![];
    for i in 0..tiles.len() - 1 {
        let a = &tiles[i];
        tiles.iter()
            .skip(i)
            .filter(|b| a.x != b.x && a.y != b.y)
            .for_each(|b| {
                let r = [
                    Tile { x: a.x.min(b.x), y: a.y.min(b.y)},   // top left
                    Tile { x: a.x.max(b.x), y: a.y.min(b.y)},   // top right
                    Tile { x: a.x.min(b.x), y: a.y.max(b.y)},   // bot left
                    Tile { x: a.x.max(b.x), y: a.y.max(b.y)},   // bot right
                ];
                rects.push(r);
            });
    }

    rects.iter()
        .filter(|r| is_inside(r, &v_edges, &h_edges))
        .fold(0, |n, r| {
            let (a, b) = (r[0], r[3]);
            let d = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
            n.max(d)
        })
}

fn is_inside(r: &[Tile;4], v_edges: &[Edge], h_edges: &[Edge]) -> bool
{
    let (a, b) = (r[0].x, r[1].x);
    let v1 = interior_intervals(r[0].y, v_edges);
    let mut v2 = boundary_intervals(r[0].y, h_edges);
    if !merge_intervals(&v1, &mut v2).into_iter()
        .any(|(x1, x2)| x1 <= a && b <= x2) { return false }

    let (a, b) = (r[2].x, r[3].x);
    let v1 = interior_intervals(r[2].y, v_edges);
    let mut v2 = boundary_intervals(r[2].y, h_edges);
    if !merge_intervals(&v1, &mut v2).into_iter()
        .any(|(x1, x2)| x1 <= a && b <= x2) { return false }

    let (a, b) = (r[0].y, r[2].y);
    let v1 = interior_intervals(r[0].x, h_edges);
    let mut v2 = boundary_intervals(r[0].x, v_edges);
    if !merge_intervals(&v1, &mut v2).into_iter()
        .any(|(x1, x2)| x1 <= a && b <= x2) { return false }

    let (a, b) = (r[1].y, r[3].y);
    let v1 = interior_intervals(r[1].x, h_edges);
    let mut v2 = boundary_intervals(r[1].x, v_edges);
    if !merge_intervals(&v1, &mut v2).into_iter()
        .any(|(x1, x2)| x1 <= a && b <= x2) { return false }

    true
}

fn merge_intervals(v1: &[u64], v2: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)>
{
    let mut v = v1.chunks(2)
        .map(|ch| (ch[0], ch[1]))
        .collect::<Vec<_>>();
    v2.append(&mut v);
    v2.sort();
    v2.dedup();

    // Merge the intervals
    let mut v3 = vec![];
    let mut curr = v2[0];
    v2.iter()
        .skip(1)
        .for_each(|&(l, r)| {
            if l <= curr.1 {
                curr.1 = curr.1.max(r);
            } else {
                v3.push(curr);
                curr = (l, r)
            }
        });
    v3.push(curr);

    v3
}

fn interior_intervals(n: u64, edges: &[Edge]) -> Vec<u64>
{
    edges.iter()
        .filter(|(_, p1, p2)| (*p1..*p2).contains(&n))
        .map(|(p, _, _)| *p)
        .collect()
}

fn boundary_intervals(a: u64, edges: &[Edge]) -> Vec<(u64, u64)>
{
    edges.iter()
        .filter(|(b, _, _)| *b == a)
        .map(|(_, p1, p2)| (*p1, *p2))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 4761736832);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 1452422268);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 50);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 24);
    }
}
