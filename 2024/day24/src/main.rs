#![allow(clippy::manual_strip)]

use std::collections::HashMap;

type Gate<'a>  = (&'a str, &'a str, char);
type Gates<'a> = HashMap<&'a str, Gate<'a>>;
type Wires<'a> = HashMap<&'a str, u64>;

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
    let (wires, gates) = load(input);
    evaluate(&wires, &gates)
}

fn part_two(input: &str) -> String
{
    let (wires, gates) = load(input);

    let mut x = 0_u64;
    let mut y = 0_u64;
    wires.iter()
        .filter(|(_, &n)| n == 1)
        .flat_map(|(w, _)| w[1..].parse::<u64>().map(|i| (w, i)))
        .for_each(|(w, i)| {
            match w.chars().next() {
                Some('x') => x |= 1 << i,
                Some('y') => y |= 1 << i,
                _ => unreachable!()
            }
        });

    // Solved manually after looking up how binary adders are implemented
    // and printing out how the z bits were directly being set and noticing
    // 4 were not like the others. However the last was due to being the last
    // and printing out the values in bites showed 31 was off which led to 
    // the "wrk" / "jrs" swap.
    let mut g = gates.clone();
    g.insert("z15", *gates.get("fph").unwrap());
    g.insert("fph", *gates.get("z15").unwrap());
    g.insert("z21", *gates.get("gds").unwrap());
    g.insert("gds", *gates.get("z21").unwrap());
    g.insert("jrs", *gates.get("wrk").unwrap());
    g.insert("wrk", *gates.get("jrs").unwrap());
    g.insert("z34", *gates.get("cqk").unwrap());
    g.insert("cqk", *gates.get("z34").unwrap());
    let z = evaluate(&wires, &g);
    assert!(z == x + y);

    "cqk,fph,gds,jrs,wrk,z15,z21,z34".into()
}

fn evaluate(wires: &Wires, gates: &Gates) -> u64
{
    let mut wires = wires.clone();
    gates.keys()
        .filter(|k| k.starts_with('z'))
        .map(|w| (w, solve(w, &mut wires, gates)))
        .filter(|(_, n)| *n == 1)
        .flat_map(|(w, _)| w[1..].parse::<u64>())
        .fold(0_u64, |z, i| z | 1 << i)
}

fn solve<'a>(w: &'a str, wires: &mut Wires<'a>, gates: &Gates<'a>) -> u64
{
    if let Some(n) = wires.get(w) {
        *n
    } else {
        let (a, b, op) = gates.get(w).unwrap();
        let a = solve(a, wires, gates);
        let b = solve(b, wires, gates);
        let n = match op {
            '&' => a & b,
            '|' => a | b,
            '^' => a ^ b,
            _ => unreachable!()
        };

        wires.insert(w, n);
        n
    }
}

fn load(input: &str) -> (Wires, Gates)
{
    let (s1, s2) = input.split_once("\n\n").unwrap();
    let wires = s1.lines()
        .filter_map(|line| line.split_once(": "))
        .flat_map(|(w, v)| v.parse::<u64>().map(|n| (w, n)))
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
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), "cqk,fph,gds,jrs,wrk,z15,z21,z34");
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
