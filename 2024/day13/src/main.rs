#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    p: (i64, i64),
}

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
    load(input).iter()
        .filter_map(play)
        .sum()
}

fn part_two(input: &str) -> i64
{
    let n = 10_000_000_000_000;
    load(input).iter()
        .map(|m| Machine { p: (m.p.0 + n, m.p.1 + n), ..*m })
        .filter_map(|m| replay(&m))
        .sum()
}

fn play(m: &Machine) -> Option<i64>
{
    // The lowest score is the highest number of B presses up to a
    // maximum of 100.
    let max_b = std::cmp::min(m.p.0 / m.b.0, m.p.1 / m.b.1).min(100);
    for b in (0..=max_b).rev() {
        let r = m.p.0 - m.b.0 * b;
        if r >= 0 && r % m.a.0 == 0 {
            let a = r / m.a.0;
            if (m.a.1 * a) + (m.b.1 * b) == m.p.1 {
                return Some(a * 3 + b)
            }
        }
    }

    None
}

fn replay(m: &Machine) -> Option<i64>
{
    // Solve two equations with two unknowns using elimination and then
    // check to see the solution is an integer value (it's fractional part
    // is 0.0).
    // We have two equations;
    // x * a.x + y * b.x = prize.x
    // x * a.y + y * b.y = prize.y
    //
    // We can scale them up to remove either b or a and get an equation with
    // a single unknown. For instance, to remove y we can scale by multiply
    // each equation by the b value of the other and then subtracting the equations.
    // We then divide by the x multplier and that'll give us the A presses.
    // We "reverse" the process to find the B presses.
    //
    // a*26 + b*67 = 10000000012748   a*x1 + b*y1 = c     a*a.0 + b*b.0 = p.0
    // a*66 + b*21 = 10000000012176   a*x2 + b*y2 = d     a*a.1 + b*b.1 = p.1
    // a*(26*21) = 10000000012748*21  (x1*y2) = (c*y2)    (a.0*b.1) = (p.0*b.1)
    // a*(66*67) = 10000000012176*67  (x2*y1) = (d*y1)    (a.1*b.0) = (p.1*b.0)
    //
    // (x1*y2) - (x2*y1) = (c*y2) - (d*y1) =>
    //    (a.0*b.1) - (a.1*b.0) = (p.0*b.1) - (p.1*b.0)
    // a = (c*y2) - (d*y1) / (x1*y2) - (x2*y1)
    // a = (p.0*b.1) - (p.1*b.0) / (a.0*b.1) - (a.1*b.0)
    //
    // a * 546  = 210000000267708
    // a * 4422 = 670000000815792
    // a * 3876 = 460000000548084
    // a = 118679050709

    let a = (m.p.0 * m.b.1 - m.p.1 * m.b.0) as f64 / (m.a.0 * m.b.1 - m.a.1 * m.b.0) as f64;
    let b = (m.p.1 * m.a.0 - m.p.0 * m.a.1) as f64 / (m.a.0 * m.b.1 - m.a.1 * m.b.0) as f64;

    if a.fract() == 0.0 && b.fract() == 0.0 {
        Some(a as i64 * 3 + b as i64)
    } else {
        None
    }
}

fn load(input: &str) -> Vec<Machine>
{
    input.split("\n\n")
        .map(|s| {
            let mut it = s.lines();

            let btn = it.next().unwrap();
            let (sx, sy) = btn[12..].split_once(", Y+").unwrap();
            let a = (sx.parse::<i64>().unwrap(), sy.parse::<i64>().unwrap());

            let btn = it.next().unwrap();
            let (sx, sy) = btn[12..].split_once(", Y+").unwrap();
            let b = (sx.parse::<i64>().unwrap(), sy.parse::<i64>().unwrap());

            let prize = it.next().unwrap();
            let (sx, sy) = prize[9..].split_once(", Y=").unwrap();
            let p = (sx.parse::<i64>().unwrap(), sy.parse::<i64>().unwrap());

            Machine { a, b, p }
        })
        .collect::<Vec<_>>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 28887);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 96979582619758);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one(input), 480);
    }
}
