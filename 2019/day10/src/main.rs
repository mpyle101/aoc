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
    let asteroids = load(input);
    let (_, count) = locate_station(&asteroids);
    count
}

fn part_two(input: &str) -> i32
{
    use std::collections::HashMap;
    use gcd::Gcd;

    type Targets = HashMap<(i32, i32), Vec<(i32, i32)>>;

    let gcd = |a: i32, b: i32| (a.unsigned_abs()).gcd(b.unsigned_abs());

    // Get the station location and the slope of all the other asteriods
    // putting their locations in a map based on the reduced slope.
    let asteroids = load(input);
    let ((x1, y1), _) = locate_station(&asteroids);
    let mut targets = asteroids.iter()
        .filter(|(x2, y2)| *x2 != x1 || *y2 != y1)
        .map(|(x2, y2)| (x2, y2, *y2 - y1, *x2 - x1))
        .fold(Targets::new(), |mut m, (&x2, &y2, dy, dx)| {
            let mut d = gcd(dy, dx) as i32;
            if d == 0 { d = 1 }
            m.entry((dy / d, dx / d)).or_default().push((x2, y2));
            m
        });

    // Sort the slope vectors by manhattan distance so we can vaporize
    // them in order. Sort the slope keys by angle. We need to flip the
    // sign of the rise because the locations are positive y down but
    // the angles are positive y up.
    targets.iter_mut().for_each(|(_, v)| v.sort_by_key(|p| md((x1, y1), *p)) );
    let mut keys = targets.keys().cloned().collect::<Vec<_>>();
    keys.sort_by(|(dy1, dx1), (dy2, dx2)| {
        let a = angle(*dx1 as f64, -*dy1 as f64);
        let b = angle(*dx2 as f64, -*dy2 as f64);
        a.partial_cmp(&b).unwrap()
    });

    // Cycle through the keys, vaporizing asteriods from the associated
    // vectors until we've zapped 200.
    let mut last = (0, 0);
    let mut count = 0;
    for (dy, dx) in keys.iter().cycle() {
        if let Some(v) = targets.get_mut(&(*dy, *dx)) {
            if !v.is_empty() {
                last = v.remove(0);
                count += 1;
            }
            if v.is_empty() {
                targets.remove(&(*dy, *dx));
            }
        }
        if count == 200 { break }
    }

    last.0 * 100 + last.1
}

fn angle(x: f64, y: f64) -> f64
{
    use std::f64::consts::PI;

    let degrees = x.atan2(y) * 180f64 / PI;
    degrees + if x < 0f64 { 360f64 } else { 0f64}
}

fn md((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u32
{
    x2.abs_diff(x1) + y2.abs_diff(y1)
}

fn locate_station(asteroids: &[(i32, i32)]) -> ((i32, i32), usize)
{
    use std::collections::HashSet;
    use gcd::Gcd;

    let gcd = |a: i32, b: i32| (a.unsigned_abs()).gcd(b.unsigned_abs());

    asteroids.iter()
        .map(|(x1, y1)| {
            let count = asteroids.iter()
                .filter(|(x2, y2)| x2 != x1 || y2 != y1)
                .map(|(x2, y2)| (y2 - y1, x2 - x1))
                .fold(HashSet::new(), |mut set, (dy, dx)| {
                    let mut d = gcd(dy, dx) as i32;
                    if d == 0 { d = 1 }
                    set.insert((dy / d, dx / d));
                    set
                })
                .len();
            ((*x1, *y1), count)
        })
        .max_by(|(_, a), (_, b)| a.cmp(b) )
        .unwrap()
}

fn load(input: &str) -> Vec<(i32, i32)>
{
    input.lines()
        .zip(0..)
        .flat_map(|(line, y)| {
            line.chars()
                .zip(0..)
                .filter(|(c, _)| *c == '#')
                .map(move |(_, x)| (x, y))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 288);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 616);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 210);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 802);
    }
}