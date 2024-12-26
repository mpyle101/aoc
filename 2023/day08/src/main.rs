use std::collections::HashMap;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1:  {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_one_alt(input);
    println!("Part 1a: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input);
    println!("Part 2:  {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two_alt(input);
    println!("Part 2a: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> u32
{
    let (inst, rest) = input.split_once("\n\n").unwrap();

    let nodes: HashMap<_, _> = rest.split('\n')
        .map(parse)
        .collect();

    let mut node = "AAA".as_bytes();
    let mut iter = inst.bytes().cycle();
    let mut steps = 0;
    while node != b"ZZZ" {
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
            let (key, dest) = parse(s);

            if key[2] == b'A' {
                keys.push(key);
                states.push(HashMap::<(usize, [u8;3]), u64>::new());
            }
            (key, dest)

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

fn part_one_alt(input: &str) -> u32
{
    let (inst, rest) = input.split_once("\n\n").unwrap();

    let mut end = 0;
    let mut start = 0;
    let mut slot = 0;
    let mut nodes = Vec::new();
    let mut slots = HashMap::new();

    rest.split('\n')
        .for_each(|line| {
            let (key, (left, right)) = parse(line);

            let left_idx = *slots.entry(left).or_insert(slot);
            if left_idx == slot { 
                slot += 1;
                nodes.push((0, 0))
            }

            let right_idx = *slots.entry(right).or_insert(slot);
            if right_idx == slot { 
                slot += 1;
                nodes.push((0, 0))
            }

            let key_idx = *slots.entry(key).or_insert(slot);
            if key_idx == slot { 
                slot += 1;
                nodes.push((left_idx, right_idx))
            } else {
                nodes[key_idx] = (left_idx, right_idx)
            }

            if key == b"AAA" { 
                start = key_idx 
            } else if key == b"ZZZ" { 
                end = key_idx 
            }
        });

    let mut idx = start;
    let mut steps = 0;
    for dir in inst.bytes().cycle() {
        steps += 1;
        idx = match dir {
            b'L' => nodes[idx].0,
            b'R' => nodes[idx].1,
               _ => panic!("Unknown instruction")
        };
        if idx == end { break }
    }

    steps
}

fn part_two_alt(input: &str) -> u64
{
    use num::integer::lcm;

    let (inst, rest) = input.split_once("\n\n").unwrap();

    let mut slot = 0;
    let mut keys = vec![];
    let mut ends = vec![];
    let mut nodes = vec![];
    let mut slots = HashMap::new();

    let mut states = vec![];
    rest.split('\n')
        .for_each(|line| {
            let (key, (left, right)) = parse(line);

            let left_idx = *slots.entry(left).or_insert(slot);
            if left_idx == slot { 
                slot += 1;
                nodes.push((0, 0))
            }

            let right_idx = *slots.entry(right).or_insert(slot);
            if right_idx == slot { 
                slot += 1;
                nodes.push((0, 0))
            }

            let key_idx = *slots.entry(key).or_insert(slot);
            if key_idx == slot { 
                slot += 1;
                nodes.push((left_idx, right_idx))
            } else {
                nodes[key_idx] = (left_idx, right_idx)
            }

            if key[2] == b'A' {
                keys.push(key_idx);
                states.push(HashMap::<(usize, usize), u64>::new());
            } else if key[2] == b'Z' {
                ends.push(key_idx)
            }

        });

    let mut iter = inst.bytes().enumerate().cycle();
    let mut res = 0;
    let mut steps = 0;
    let mut remove = Vec::with_capacity(keys.len());

    while !keys.is_empty() {
        steps += 1;
        let (idx, dir) = iter.next().unwrap();
        for idx in keys.iter_mut() {
            let (l, r) = nodes[*idx];
            *idx = match dir {
                b'L' => l,
                b'R' => r,
                   _ => panic!("Unknown instruction: {dir}")
            };
        }

        for (i, key) in keys.iter().enumerate() {
            if ends.contains(key) {
                let map = &mut states[i];
                if let Some(n) = map.insert((idx, *key), steps) {
                    let cycle = steps - n;
                    res = if res == 0 { cycle } else { lcm(res, cycle) };
                    remove.push(i);
                }
            }
        }

        remove.sort_by(|a, b| b.cmp(a));
        remove.iter()
            .for_each(|&i| {
                keys.remove(i);
                states.remove(i);
            });
        remove.clear();
    }

    res
}

fn parse(line: &str) -> (&[u8], (&[u8], &[u8]))
{
    let mut iter = line.split(' ');
    let key = iter.next().unwrap().as_bytes();
    iter.next();    // =
    let left = &iter.next().unwrap().as_bytes()[1..4];
    let right = &iter.next().unwrap().as_bytes()[0..3];
        
    (key, (left, right))
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
    fn input_part_one_alt()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one_alt(input), 18673);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input), 17972669116327);
    }

    #[test]
    fn input_part_two_alt()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two_alt(input), 17972669116327);
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
