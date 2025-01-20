use ndarray::Array2;

type Rect = (i32, i32, i32, i32);
type Cmd  = (char, Rect);

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
    use ndarray::s;

    let to_slice = |(x1, y1, x2, y2): Rect| s![x1..=x2, y1..=y2];

    let cmds = load(input);
    cmds.iter()
        .map(|(c, r)| (c, to_slice(*r)))
        .fold(
            Array2::<u8>::zeros((1000, 1000)),
            |mut m, (c, sl)| {
                let mut n = m.slice_mut(sl);
                match c {
                    '+' => n.iter_mut().for_each(|v| *v = 1),
                    '!' => n.iter_mut().for_each(|v| *v = 0),
                    '-' => n.iter_mut().for_each(|v| *v = (*v + 1) % 2),
                     _  => unreachable!()
                };
                m
            }
        )
        .fold(0, |acc, &v| acc + v as i32)
}

fn part_two(input: &str) -> i32
{
    use ndarray::s;

    let to_slice = |(x1, y1, x2, y2): Rect| s![x1..=x2, y1..=y2];

    let cmds = load(input);
    cmds.iter()
        .map(|(c, r)| (c, to_slice(*r)))
        .fold(
            Array2::<u8>::zeros((1000, 1000)),
            |mut m, (c, sl)| {
                let mut n = m.slice_mut(sl);
                match c {
                    '+' => n.iter_mut().for_each(|v| *v += 1),
                    '!' => n.iter_mut().for_each(|v| *v += 2),
                    '-' => n.iter_mut().for_each(|v| if *v > 0 { *v -= 1 }),
                     _  => unreachable!()
                };
                m
            }
        )
        .fold(0, |acc, &v| acc + v as i32)
}

fn load(input: &str) -> Vec<Cmd>
{
    input.lines()
        .map(|line| line.split(' '))
        .map(|mut iter| {
            if let Some("turn") = iter.next() {
                match iter.next() {
                    Some("on")  => ('+', get_rect(iter)),
                    Some("off") => ('-', get_rect(iter)),
                    _ => unreachable!()
                }
            } else {
                ('!', get_rect(iter))
            }
        })
        .collect()
}

fn get_rect<'a>(mut iter: impl Iterator<Item=&'a str>) -> Rect
{
    let s = iter.next().unwrap();
    let (x1, y1) = s.split_once(',').unwrap();
    let x1 = x1.parse::<i32>().unwrap();
    let y1 = y1.parse::<i32>().unwrap();

    iter.next();    // "through"

    let s = iter.next().unwrap();
    let (x2, y2) = s.split_once(',').unwrap();
    let x2 = x2.parse::<i32>().unwrap();
    let y2 = y2.parse::<i32>().unwrap();

    (x1, y1, x2, y2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one() {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 543903);
    }

    #[test]
    fn input_part_two() {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 14687245);
    }
}