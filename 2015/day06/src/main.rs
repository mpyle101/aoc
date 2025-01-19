use ndarray::{Array2, SliceInfo, SliceInfoElem, Dim};

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
    let cmds = load(input);
    cmds.iter()
        .map(|cmd| (cmd, cmd.slice()))
        .fold(
            Array2::<u8>::zeros((1000, 1000)),
            |mut m, (cmd, sl)| {
                let mut n = m.slice_mut(sl);
                match cmd {
                    Cmd::On(_)  => { n.iter_mut().for_each(|v| *v = 1); m },
                    Cmd::Off(_) => { n.iter_mut().for_each(|v| *v = 0); m },
                    Cmd::Tog(_) => { n.iter_mut().for_each(|v| *v = (*v + 1) % 2); m },
                }
            }
        )
        .fold(0, |acc, &v| acc + v as i32)
}

fn part_two(input: &str) -> i32
{
    let cmds = load(input);
    cmds.iter()
        .map(|cmd| (cmd, cmd.slice()))
        .fold(
            Array2::<u8>::zeros((1000, 1000)),
            |mut m, (cmd, sl)| {
                let mut n = m.slice_mut(sl);
                match cmd {
                    Cmd::On(_)  => { n.iter_mut().for_each(|v| *v += 1); m }
                    Cmd::Tog(_) => { n.iter_mut().for_each(|v| *v += 2); m },
                    Cmd::Off(_) => { n.iter_mut().for_each(|v| if *v > 0 { *v -= 1 }); m },
                }
            }
        )
        .fold(0, |acc, &v| acc + v as i32)
}

type Rect = ((i32, i32), (i32, i32));
type Slice = SliceInfo<[SliceInfoElem; 2], Dim<[usize; 2]>, Dim<[usize; 2]>>;

enum Cmd {
    On(Rect),
    Off(Rect),
    Tog(Rect),
}

impl Cmd {
    fn slice(&self) -> Slice
    {
        use ndarray::s;

        match self {
            Cmd::On((p1, p2))  => s![p1.0..=p2.0, p1.1..=p2.1],
            Cmd::Off((p1, p2)) => s![p1.0..=p2.0, p1.1..=p2.1],
            Cmd::Tog((p1, p2)) => s![p1.0..=p2.0, p1.1..=p2.1],
        }
    }
}

fn load(input: &str) -> Vec<Cmd>
{
    input.lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|v| match v[1] {
            "on"  => Cmd::On(make_rect(v[2], v[4])),
            "off" => Cmd::Off(make_rect(v[2], v[4])),
            _     => Cmd::Tog(make_rect(v[1], v[3])),
        })
        .collect()
}

fn make_rect(pt1: &str, pt2: &str) -> Rect
{
    let v1: Vec<_> = pt1.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    let v2: Vec<_> = pt2.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    ((v1[0], v1[1]), (v2[0], v2[1]))
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