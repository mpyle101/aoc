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
    let (_, p) = input.lines()
        .map(|line| {
            let c = line.chars().next().unwrap();
            let n = line[1..].parse::<i32>().unwrap();
            (c, n)
        })
        .fold(('E', (0, 0)), |(d, p), (c, n)| {
            match c {
                'F'             => (d, advance(d, n, p)),
                'R'|'L'         => (rotate(d, c, n), p),
                'N'|'S'|'E'|'W' => (d, advance(c, n, p)),
                _ => unreachable!()
            }
        });

    md((0, 0), p)
}

fn part_two(input: &str) -> u32
{
    let (p, _) = input.lines()
        .map(|line| {
            let c = line.chars().next().unwrap();
            let n = line[1..].parse::<i32>().unwrap();
            (c, n)
        })
        .fold(((0, 0), (10, 1)), |(sp, wp), (c, n)| {
            match c {
                'F'             => move_ship(n, sp, wp),
                'R'|'L'         => (sp, rotate_wp(c, n, sp, wp)),
                'N'|'S'|'E'|'W' => (sp, advance(c, n, wp)),
                _ => unreachable!()
            }
        });

    md((0, 0), p)
}

fn move_ship(
    n: i32,
    (sx, sy): (i32, i32),   // sp
    (wx, wy): (i32, i32)    // wp
) -> ((i32, i32), (i32, i32))
{
    let dx = wx - sx;
    let dy = wy - sy;
    let sp = (sx + dx * n, sy + dy * n);
    let wp = (sp.0 + dx, sp.1 + dy);

    (sp, wp)
}

fn rotate_wp(d: char, n: i32, (sx, sy): (i32, i32), (wx, wy): (i32, i32)) -> (i32, i32)
{
    let n = n / 90;
    let p = (wx - sx, wy - sy);
    let (x, y) = if d == 'R' {
        (0..n).fold(p, |(x, y), _| (y, -x))
    } else {
        (0..n).fold(p, |(x, y), _| (-y, x))
    };

    (x + sx, y + sy)
}

fn md((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u32
{
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

static DIRS: [char;4] = ['E', 'S', 'W', 'N'];

fn rotate(d: char, c: char, n: i32) -> char
{
    let n = n / 90;
    let i = match d {
        'E' => 0,
        'S' => 1,
        'W' => 2,
        'N' => 3,
         _  => unreachable!()
    };
    let a = if c == 'R' { i + n } else { i - n };
    let i = a.rem_euclid(4) as usize;

    DIRS[i]
}

fn advance(d: char, n: i32, (x, y): (i32, i32)) -> (i32, i32)
{
    match d {
        'N' => (x, y + n),
        'S' => (x, y - n),
        'E' => (x + n, y),
        'W' => (x - n, y),
         _  => unreachable!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 562);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 101860);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 25);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 286);
    }
}