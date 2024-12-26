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

fn part_one(input: &str) -> i64
{
    let mut p = (0, 0);
    let path: Vec<_> = input.lines()
        .fold(vec![(0 ,0)], |mut v, line| {
            let mut iter = line.split(' ');
            let dir = iter.next().unwrap().chars().next().unwrap();
            let n = iter.next().map(|s| s.parse().unwrap()).unwrap();
            p = dig(p, dir, n);
            v.push(p);
            v
        });

    fill(&path)
}

#[allow(dead_code)]
fn part_two(input: &str) -> i64
{
    let mut p = (0, 0);
    let path: Vec<_> = input.lines()
        .fold(vec![(0 ,0)], |mut v, line| {
            let mut iter = line.split(' ');
            iter.next();    // dir
            iter.next();    // n
            let mut code = iter.next().unwrap();
            code = &code[2..code.len()-1];
            let n = i64::from_str_radix(&code[0..5], 16).unwrap();
            let dir = match code.as_bytes()[5] {
                b'0' => 'R',
                b'1' => 'D',
                b'2' => 'L',
                b'3' => 'U',
                _ => panic!("Unkwown direction: {code}")
            };
            p = dig(p, dir, n);    
            v.push(p);
            v
        });

    fill(&path)
}

fn dig((x, y): (i64, i64), dir: char, n: i64) -> (i64, i64)
{
    if dir == 'U' {
        (x, y - n)
    } else if dir == 'D' {
        (x, y + n)
    } else if dir == 'L' {
        (x - n, y)
    } else {
        (x + n, y)
    }
}

fn fill(path: &[(i64, i64)]) -> i64
{
    let mut h_edges: Vec<_> = path.windows(2)
        .map(|ch| (ch[0], ch[1]))
        .filter(|(p1, p2)| p1.1 == p2.1)
        .map(|(p1, p2)| if p1.0 < p2.0 { (p1, p2) } else { (p2, p1) })
        .collect();
    h_edges.sort_by(|p1, p2| p1.1.1.cmp(&p2.1.1));

    let mut v_lines: Vec<_> = path.windows(2)
        .map(|ch| (ch[0], ch[1]))
        .filter(|(p1, p2)| p1.1 != p2.1)
        .map(|(p1, p2)| if p1.1 < p2.1 { (p1, p2) } else { (p2, p1) })
        .map(|(p1, _)| p1.0)
        .collect();
    v_lines.sort();
    v_lines.dedup();

    let rects: Vec<_> = v_lines.windows(2)
        .map(|v| (v[0], v[1]))
        .flat_map(|(x_min, x_max)| {
            let edges: Vec<_> = h_edges.iter()
                .filter(|e| e.0.0 <= x_min && e.1.0 >= x_max)
                .collect();
            edges.chunks(2)
                .map(|e| (e[0], e[1]))
                .map(|(e1, e2)| ((x_min, e1.0.1), (x_max, e2.0.1)))
                .collect::<Vec<_>>()
        })
        .collect();

    let area: i64 = rects.iter()
        .map(|(tl, br)| (br.0 - tl.0 + 1) * (br.1 - tl.1 + 1))
        .sum();

    area - (0..rects.len() - 1)
        .map(|i| {
            let a = rects[i];
            rects.iter()
                .skip(i + 1)
                .filter_map(|b| intersection(a, *b))
                .map(|r| (r.1.0 - r.0.0 + 1) * (r.1.1 - r.0.1 + 1))
                .sum::<i64>()
        })
        .sum::<i64>()
}

type Rect = ((i64, i64), (i64, i64));
fn intersection(
    ((x1, y1), (x2, y2)): Rect,
    ((x3, y3), (x4, y4)): Rect
) -> Option<Rect>
{
    let x5 = x1.max(x3);
    let x6 = x2.min(x4);
    let y5 = y1.max(y3);
    let y6 = y2.min(y4);

    if x5 <= x6 && y5 <= y6 {
        Some(((x5, y5), (x6, y6)))
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 62573);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 54_662_804_037_719);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 62);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 952_408_144_115);
    }
}
