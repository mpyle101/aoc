fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one::<1000>(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one<const N: usize>(input: &str) -> usize
{
    let (pairs, boxes) = load(input);
    let mut circuits = (0..boxes.len())
        .map(|i| vec![i])
        .collect::<Vec<_>>();
    
    pairs.iter()
        .take(N)
        .for_each(|&(_, a, b)| {
            let i = circuits.iter().position(|v| v.contains(&a)).unwrap();
            let j = circuits.iter().position(|v| v.contains(&b)).unwrap();
            if i != j {
                let mut v = circuits[j].clone();
                circuits[i].append(&mut v);
                circuits.remove(j);
            }
        });
    circuits.sort_by_key(|v| std::cmp::Reverse(v.len()));

    circuits.iter()
        .take(3)
        .map(|v| v.len())
        .product()
}

fn part_two(input: &str) -> i64
{
    let (pairs, boxes) = load(input);
    let mut circuits = (0..boxes.len())
        .map(|i| vec![i])
        .collect::<Vec<_>>();

    let mut last = (0, 0);
    for (_, a, b) in pairs {
        let i = circuits.iter().position(|v| v.contains(&a)).unwrap();
        let j = circuits.iter().position(|v| v.contains(&b)).unwrap();
        if i != j {
            let mut v = circuits[j].clone();
            circuits[i].append(&mut v);
            circuits.remove(j);
        }
        last = (a, b);
        if circuits.len() == 1 { break }
    }

    boxes[last.0].0 * boxes[last.1].0
}

type Pos = (i64, i64, i64);

#[allow(clippy::needless_range_loop)]
fn load(input: &str) -> (Vec<(i64, usize, usize)>, Vec<Pos>)
{
    let boxes = input.lines()
        .map(|l| {
            let mut iter = l.split(',');
            let x = iter.next().unwrap().parse::<i64>().unwrap();
            let y = iter.next().unwrap().parse::<i64>().unwrap();
            let z = iter.next().unwrap().parse::<i64>().unwrap();

            (x, y, z)
        })
        .collect::<Vec<_>>();

    let mut pairs = vec![];
    for a in 0..boxes.len() - 1 {
        let (ax, ay, az) = boxes[a];
        for b in a + 1..boxes.len() {
            let (bx, by, bz) = boxes[b];
            let d = (ax - bx).pow(2) + (ay - by).pow(2) + (az - bz).pow(2);
            pairs.push((d, a, b));
        }
    }
    pairs.sort();

    (pairs, boxes)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one::<1000>(input), 75582);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 59039696);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_one::<10>(input), 40);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example.txt");
        assert_eq!(part_two(input), 25272);
    }
}
