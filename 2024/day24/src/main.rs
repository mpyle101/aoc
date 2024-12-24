#![allow(clippy::manual_strip)]

use std::collections::HashMap;

type Wires<'a> = HashMap<&'a str, u64>;
type Gates<'a> = HashMap<&'a str, (&'a str, &'a str, char)>;

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

fn part_one(input: &str) -> u64
{
    let (mut wires, gates) = load(input);
    gates.keys()
        .filter(|k| k.starts_with('z'))
        .map(|w| (w, evaluate(w, &mut wires, &gates)))
        .filter(|(_, n)| *n == 1)
        .filter_map(|(w, _)| w[1..].parse::<u64>().ok())
        .fold(0_u64, |z, i| z | 1 << i)
}

fn part_two(input: &str) -> String
{
    let (mut wires, gates) = load(input);
    let z_wires = gates.keys()
        .filter(|k| k.starts_with('z'))
        .collect::<Vec<_>>();

    let mut x = 0_u64;
    let mut y = 0_u64;
    wires.iter()
        .filter(|(_, &n)| n == 1)
        .filter_map(|(w, _)| w[1..].parse::<u64>().ok().map(|i| (w, i)))
        .for_each(|(w, i)| {
            match w.chars().next() {
                Some('x') => x |= 1 << i,
                Some('y') => y |= 1 << i,
                _ => unreachable!()
            }
        });

    let z = z_wires.iter()
        .map(|w| (w, evaluate(w, &mut wires, &gates)))
        .filter(|(_, n)| *n == 1)
        .filter_map(|(w, _)| w[1..].parse::<u64>().ok())
        .fold(0_u64, |z, i| z | 1 << i);
    
    println!("{x:046b} {x}");
    println!("{y:046b} {y}");
    println!("{:046b} {:?}", x + y, x + y);
    println!("{z:046b} {z}");

    "nope".into()
}

fn evaluate<'a>(w: &'a str, wires: &mut Wires<'a>, gates: &Gates<'a>) -> u64
{
    if let Some(n) = wires.get(w) {
        *n
    } else if let Some(&(a, b, op)) = gates.get(w) {
        let a = evaluate(a, wires, gates);
        let b = evaluate(b, wires, gates);
        let n = match op {
            '&' => a & b,
            '|' => a | b,
            '^' => a ^ b,
            _ => unreachable!()
        };

        wires.insert(w, n);
        n
    } else {
        unreachable!()
    }
}

fn load(input: &str) -> (Wires, Gates)
{
    let (s1, s2) = input.split_once("\n\n").unwrap();
    let wires = s1.lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(w, v)| v.parse::<u64>().ok().map(|n| (w, n)))
        .collect::<Wires>();

    let gates = s2.lines()
        .fold(Gates::new(), |mut m, line| {
            let mut it = line.split(' ');
            let a  = it.next().unwrap();
            let op = match it.next().unwrap() {
                "AND" => '&',
                "OR"  => '|',
                "XOR" => '^',
                _ => unreachable!()
            };
            let b  = it.next().unwrap();
            it.next();
            let c  = it.next().unwrap();

            m.insert(c, (a, b, op));
            m
        });

    (wires, gates)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 46463754151024);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 4);

        let input = include_str!("../example2.txt");
        assert_eq!(part_one(input), 2024);
    }
}
