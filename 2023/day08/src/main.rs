use std::collections::HashMap;

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
    let (inst, rest) = input.split_once("\n\n").unwrap();

    let nodes: HashMap<_, _> = rest.split('\n')
        .map(|s| {
            let mut iter = s.split(' ');
            let key = iter.next().unwrap();
            iter.next();    // =
            let left = &iter.next().unwrap()[1..4];
            let right = &iter.next().unwrap()[0..3];

            (key, (left, right))

        })
        .collect();

    let mut node = "AAA";
    let mut iter = inst.bytes().cycle();
    let mut steps = 0;
    while node != "ZZZ" {
        steps += 1;
        let dir = iter.next().unwrap();
        let (l, r) = nodes.get(node).unwrap();
        node = match dir {
            b'L' => l,
            b'R' => r,
               _ => panic!("Unknown instruction: {dir}")
        };
    }

    steps
}

fn part_two(input: &str) -> u64
{
    use num::integer::lcm;

    let (inst, rest) = input.split_once("\n\n").unwrap();

    let mut keys = vec![];
    let mut states = vec![];
    let nodes: HashMap<_, _> = rest.split('\n')
        .map(|s| {
            let mut iter = s.split(' ');
            let key = iter.next().unwrap().as_bytes();
            iter.next();    // =
            let left = &iter.next().unwrap().as_bytes()[1..4];
            let right = &iter.next().unwrap().as_bytes()[0..3];

            if key[2] == b'A' {
                keys.push(key);
                states.push(HashMap::<(usize, [u8;3]), u64>::new());
            }
            (key, (left, right))

        })
        .collect();

    let mut iter = inst.bytes().enumerate().cycle();
    let mut steps = 0u64;
    let mut cycles = vec![0u64;keys.len()];

    let mut found = false;
    while !found && cycles.iter().any(|n| *n == 0) {
        steps += 1;
        let (idx, dir) = iter.next().unwrap();
        for key in keys.iter_mut() {
            let (l, r) = nodes.get(key).unwrap();
            *key = match dir {
                b'L' => *l,
                b'R' => *r,
                   _ => panic!("Unknown instruction: {dir}")
            };
        }

        for (i, key) in keys.iter().enumerate() {
            if key[2] == b'Z' && cycles[i] == 0 {
                let map = &mut states[i];
                let mut k = [0u8;3];
                (0..3).for_each(|i| { k[i] = key[i]; });
                if let Some(n) = map.insert((idx, k), steps) {
                    cycles[i] = steps - n;
                }
            }
        }

        found = keys.iter().all(|key| key[2] == b'Z');
    }

    if found {
        steps
    } else {
        let first = cycles[0];
        cycles.iter().fold(first, |a, b| lcm(a, *b))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 18673);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 17972669116327);
    }

    #[test]
    fn example_part_one()
    {
        let input = include_str!("../example1.txt");
        assert_eq!(part_one(input), 2);
    }

    #[test]
    fn example_part_two()
    {
        let input = include_str!("../example2.txt");
        assert_eq!(part_two(input), 6);
    }
}
